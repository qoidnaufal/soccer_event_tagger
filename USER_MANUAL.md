## User Manual [eng]
### Register match info
After opening the app, first thing you wanna do is to register match info. Click menu button <img src="public/buttons/menu.svg" alt="menu" width="15" height="15"> on the upper left, select `Register Match Info`.
You will see this on the screen:
![register match info](public/screenshots/register_match_info.png)
1. Pick `date and time` of the match,
2. Insert the `team name`, both home and away,
3. Insert the `list of the players`, both home and away. The rule is as follow:
  * [`number`][`)`][`player name`][`/position`] if the player is registered as starting xi. The currently available positions to register are: gk, rfb, lfb, cb, mf, rwg, lwg, cf,
  * If the player is not registered as starting xi, just ommit [`/position`],
  * Each player must be separated by comma (`,`),
  * For example: `7) Marselino Ferdinan /mf` means Marselino will be registered as starting xi in which he started as a Midfielder,
4. Click `submit`, and back to home screen by clicking the top left button <img src="public/buttons/home.svg" alt="home" width="15" height="15">

### Back to home screen
![home screen](public/screenshots/homescreen.png)
  1. Select the `team sheet` that you have registered,
  2. The list of the registered players will appear, unfortunately the list of the players is still sorted by the player's number, not by whether they start the game or not,
  3. The players  with `/position` will automatically registered as `events` with event name: `Play`, event type: `Start`. I designed it this way to make it easier later when working with the data (eg: calculating minutes played, etc.),

### Tagging the events
Before you start tagging the events, click on the menu and select `Open Video`. The currently supported video format is *.mp4 only.
You can play / pause the video by pressing `space` on your keyboard. You can also navigate by pressing `right` or `left` on your keyboard. You can also speed up the video by pressing `Ctrl +` or `Ctrl 2`, reduce the playback speed by pressing `Ctrl -`, or return to normal speed by pressing `Ctrl 1`.
![buffer](public/screenshots/buffer.png)
1. At the start of the event, pause the video, click anywhere on the football pitch where the start of an event occured. Your click will record the `x` & `y` coordinate of the pitch. Press `Shift S` on your keyboard to register it as the location of the start of an event. Keep in mind, when you press `Shift S` this will also register the time of the video.
2. To register the type of the event, type on your keyboard with the following format:
  * [`team state`][`player number`][`/`][`event`],
  * `team state` => is either `h` = `Home` or `a` = `Away`. This will directly query the team name you registered earlier. For example if you registered PSID Jombang as team away, when you type a it will show PSID Jombang,
  * `player number` => is the number of the player you registered
  * `event` => you can check this on the `menu` and select `Shortcuts Info`. At the moment, i'm still working on making this page easier to navigate because there are a lot of shortcuts you can use. But basically, the shortcut itself should be pretty intuitive, like `ps` stands for `pass success`, `dgl` stands for `duel ground lose`, etc.

Some events occured as "long event" in which they started somehere and ended somewhere else, eg: passes, shots and dribbles. These type of events need to be registered with the starting time & starting location of the event, and the end time and end location of the event. To register the end location of the event, pick the coordinate and press `Shift E`. Make sure you register the end of the event on the correct video time. Other events just occured as "instant event", eg: aerial duel, foul, etc. With these type of events, you just need to register the start of the event.

Some events also occured as "displacement of the ball", in which there is another player at the end of the event, eg: pass (another player either receive the ball or intercept it). You need to add additional arguments with this format: [`/`][`team state`][`player number`].

Example:
  * `a4/daw` => `player no.[4]` from `team [a]way`, event name: `[d]uel`, event type: `[a]erial`, outcome: `[w]on`.
  * `h8/pi/a6` => `player no.[8]` from `team [h]ome`, event name: `[p]ass`, event type: `open play`, outcome: `[i]ntercepted`, interceptor: `player no.[6]` from `team [a]way`

I designed this way to make it easier later when processing the data. For example, within the registered pass event, there is also the registered pass receiver or pass interceptor.

Clicking on the row of the table will navigate to the minute of the start of the selected event. You can also delete each individual row by clicking `x` button on the left most of the row. If you click <img src="public/buttons/delete.svg" alt="delete" width="15" height="15"> button, all the registered events will be deleted.

### Export the data
Go to menu, select `Export Data`. For now, only *.csv is available.

## Panduan Penggunaan
### Pendafataran info pertandingan
Hal pertama yang perlu dilakukan setelah aplikasi terbuka adalah registrasi info pertandingan. Tekan tombol menu <img src="public/buttons/menu.svg" alt="menu" width="15" height="15"> di pojok kiri atas, pilih `Register Match Info`.
Anda akan dinavigasi ke halaman dengan tampilan seperti ini:
![register match info](public/screenshots/register_match_info.png)
1. Pilih `tanggal dan waktu` pertandingan,
2. Masukkan `nama tim`, kandang dan tandang,
3. Masukkan `daftar nama pemain`, kandang dan tandang. Aturan penulisannya sebagai berikut:
  * [`nomor`][`)`][`nama pemain`][`/posisi`] kalau pemainnya bermain sebagai starter. Untuk sekarang, posisi yang bisa didaftarkan yaitu: gk, rfb, lfb, cb, mf, rwg, lwg, cf,
  * Untuk pemain yang tidak didaftarkan sebagai starter, tidak perlu ditambahkan [`/posisi`],
  * Setiap pemain dipisahkan dengan (`,`),
  * Contoh: `7) Marselino Ferdinan /mf` berarti Marselino akan didaftarkan sebagai starter dengan posisi *midfielder* [`MF`],
4. Setelah selesai tekan `submit`,
5. Kembali ke laman awal dengan menekan tombol <img src="public/buttons/home.svg" alt="home" width="15" height="15"> di pojok kiri atas

### Kembali ke laman awal
![home screen](public/screenshots/homescreen.png)
  1. Tekan tombol *dropdown* di pojok kanan atas untuk memilih `info pertandingan` yang tadi sudah didaftarkan,
  2. Daftar pemain yang diregistrasi akan muncul. Untuk saat ini, daftar pemain diurutkan berdasar nomor punggung yang didaftarkan, bukan berdasarkan apakah pemain tersebut bermain starter atau tidak,
  3. Pemain yang didaftarkan sebagai starter (pemain dengan `/posisi` saat pendaftaran info pertandingan), secara otomatis akan didaftarkan di `*event*` sebagai *event*_name: `Play`, *event* type: `Start`. Hal ini untuk memudahkan nantinya ketika memproses data (misal: menghitung menit main, dll.),

### Menandai *event*
Sebelum mulai menandai *event*, tekan tombol menu dan pilih `Open Video`. Saat ini aplikasi hanya dapat membuka file *.mp4.
Anda bisa memainkan / menghentikan video dengan menekan `spasi` di *keyboard*. Anda juga bisa menjalankan video ke depan atau ke belakang dengan menekan `kanan` atau `kiri` di *keyboard*. Anda juga bisa mempercepat pemutaran video dengan menekan `Ctrl +` atau `Ctrl 2`, memperlambat pemutaran video dengan menekan `Ctrl -`, atau kembali ke kecepatan pemutaran normal dengan menekan `Ctrl 1` di *keyboard*.
![buffer](public/screenshots/buffer.png)
1. Pada permulaan *event*, hentikan video, tekan tombol *mouse* kiri di gambar lapangan di mana lokasi *event* terjadi untuk merekam koordinat `x` & `y`. Tekan `Shift S` di *keyboard* untuk mendaftarkan lokasi tersebut sebagai lokasi awal dari *event*. Ketika menekan `Shift S` waktu dari video juga akan didaftarkan sebagai waktu awal dari event.
2. Untuk mendaftarkan jenis *event*, ketik di *keyboard* dengan format sebagai berikut:
  * [`h atau a`][`nomor`][`/`][`event`],
  * `h` = `Home` (kandang) atau `a` = `Away` (tandang) akan langsung mencocokkan dengan nama tim yang didaftarkan. Contoh, jika anda mendaftarkan PSID Jombang sebagai tim tandang, ketika andang tekan `a` akan muncul nama PSID Jombang,
  * `nomor` => nomor punggung dari pemain yang didaftarkan
  * `event` => anda bisa cek dengan cara tekan tombol `menu` pilih `Shortcuts Info`. Untuk sementara, saya masih tampilkan kodenya karena ada banyak pintasan *event* yang bisa digunakan. Tapi pada dasarnya, pintasan yang digunakan cukup intuitif, misal `ps` adalah `pass success`, `dgl` adalah `duel ground lose`, dll. Tampilan yang lebih mudah untuk dinavigasi masih sedang dalam pengerjaan.

Beberapa *event* terjadi sebagai "*event* panjang" di mana event tersebut bermula di satu titik dan berakhir di titik lain, contoh: *passes*, *shots* dan *dribbles*. Tipe event panjang harus didaftarkan dengan titik dan waktu awal, juga titik dan waktu akhir. Untuk mendaftarkan titik dan waktu akhir, pilih koordinat di lapangan lalu tekan `Shift E`. Anda harus memastikan waktu yang terdaftar tepat.

Sedangkan beberapa *event* lainnya terjadi sebagai "*event* instan", contoh: *aerial duel*, *foul*, dll. Anda cukup mendaftarkan titik dan waktu awal dari *event* tersebut. Khusus untuk pergantian, kartu kuning dan merah tidak perlu mendaftarkan titik lokasinya.

Beberapa *event* juga terjadi sebagai "perpindahan bola", di mana terdapat pemain lain di titik akhir dari *event* tersebut, misal: *pass* (di titik akhir ada pemain yang menerima atau melakukan intersep). Anda perlu menambahkan pemain di akhir dari event tersebut dengan format: [`/`][`a atau h`][`nomor`].

Contoh:
  * `a4/daw` => `pemain nomor [4]` dari `tim [a]way`, *event name*: `[d]uel`, *event type*: `[a]erial`, *outcome*: `[w]on`.
  * `h8/pi/a6` => `pemain nomor [8]` dari `tim [h]ome`, *event name*: `[p]ass`, *event type*: `open play`, *outcome*: `[i]ntercepted`, di titik akhir: `pemain nomor [6]` dari `tim [a]way`

Saya mendesainnya seperti ini untuk memudahkan nantinya dalam memproses data. Sebagai contoh, dalam *event* pass yang terdaftar, sudah ada data siapa yang menerima atau melakukan intersep.

Menekan baris pada tabel akan menavigasi ke menit video dari *event* yang dipilih. Anda juga bisa menghapus baris dari tabel denga cara menekan tombol `x` di bagian paling kiri dari baris tersebut. Menekan tombol <img src="public/buttons/delete.svg" alt="delete" width="15" height="15"> akan menghapus seluruh *event* yang telah didaftarkan.

### Penyimpanan data
Untuk saat ini data yang diambil masih belum disimpan secara permanen, sehingga ketika anda menutup aplikasi, data akan terhapus. Anda bisa mengekspor data yang sudah anda ambil dengan cara tekan menu, pilih `Export Data`. Untuk sementara hanya bisa mengekspor ke format *.csv.
Pada versi selanjutnya dari aplikasi ini, data yang telah diambil akan disimpan secara permanen.
