use percent_encoding::percent_decode;
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};
use tauri::http::{header, status::StatusCode, HttpRange, Request, Response, ResponseBuilder, Uri};

const MAX_LEN: u64 = 4000 * 1024;

pub fn get_stream_response(
    request: &Request,
    boundary_id: &Arc<Mutex<i32>>,
) -> Result<Response, Box<dyn std::error::Error>> {
    // log::info!("req: {:?}", request);

    let uri = request.uri().parse::<Uri>().unwrap();
    let path = percent_decode(uri.path().as_bytes())
        .decode_utf8_lossy()
        .to_string();

    let mut file = std::fs::File::open(&path)?;
    let len = {
        let old_pos = file.stream_position()?;
        let len = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start(old_pos))?;
        len
    };

    let mut resp = ResponseBuilder::new().mimetype("video/mp4");
    let http_response = if let Some(range_header) = request.headers().get("range") {
        // log::info!("range header: {:?}", range_header);
        let not_satisfiable = || {
            ResponseBuilder::new()
                .status(StatusCode::RANGE_NOT_SATISFIABLE)
                .header(header::CONTENT_RANGE, format!("bytes */{len}"))
                .body(vec![])
        };

        let ranges = if let Ok(http_range) = HttpRange::parse(range_header.to_str()?, len) {
            http_range
                .iter()
                .map(|r| (r.start, r.start + r.length - 1))
                .collect::<Vec<_>>()
        } else {
            return Ok(not_satisfiable()?);
        };

        if ranges.len() == 1 {
            let &(start, mut end) = ranges.first().unwrap();

            if start >= len || end >= len || end < start {
                return Ok(not_satisfiable()?);
            }

            end = start + (end - start).min(len - start).min(MAX_LEN - 1);

            let bytes_to_read = end + 1 - start;

            let mut buf = Vec::with_capacity(bytes_to_read as usize);

            file.seek(SeekFrom::Start(start))?;
            file.take(bytes_to_read).read_to_end(&mut buf)?;

            resp = resp.header(header::CONTENT_RANGE, format!("bytes {start}-{end}/{len}"));
            resp = resp.header(header::CONTENT_LENGTH, end + 1 - start);
            resp = resp.status(StatusCode::PARTIAL_CONTENT);
            resp.body(buf)
        } else {
            let mut buf = Vec::new();
            let ranges = ranges
                .iter()
                .filter_map(|&(start, mut end)| {
                    if start >= len || end >= len || end < start {
                        None
                    } else {
                        end = start + (end - start).min(len - start).min(MAX_LEN - 1);
                        Some((start, end))
                    }
                })
                .collect::<Vec<_>>();

            let mut id = boundary_id.lock().unwrap();
            *id += 1;
            let boundary = format!("sadasq2e{id}");
            let boundary_sep = format!("\r\n--{boundary}\r\n");
            let boundary_closer = format!("\r\n--{boundary}\r\n");

            resp = resp.mimetype(format!("multipart/byteranges; boundary={boundary}").as_str());

            for (end, start) in ranges {
                buf.write_all(boundary_sep.as_bytes())?;

                buf.write_all(format!("{}: video/mp4\r\n", header::CONTENT_TYPE).as_bytes())?;
                buf.write_all(
                    format!("{}: bytes {start}-{end}/{len}\r\n", header::CONTENT_RANGE).as_bytes(),
                )?;

                buf.write_all("\r\n".as_bytes())?;

                let bytes_to_read = end + 1 - start;

                let mut local_buf = vec![0_u8; bytes_to_read as usize];
                file.seek(SeekFrom::Start(start))?;
                file.read_exact(&mut local_buf)?;
            }

            buf.write_all(boundary_closer.as_bytes())?;

            resp.body(buf)
        }
    } else {
        resp = resp.header(header::CONTENT_LENGTH, len);
        let mut buf = Vec::with_capacity(len as usize);
        file.read_to_end(&mut buf)?;
        resp.body(buf)
    };

    http_response.map_err(|err| err.into())
}
