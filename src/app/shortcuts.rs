use leptos::*;
use leptos_router::A;

const TEXT_HIGHLIGHT: &str = "text-red-600";
const SUBTITLE_BAR: &str = "bg-slate-800 pl-1 text-white";

#[component]
pub fn Shortcuts() -> impl IntoView {
    view! {
        <div class="text-xs absolute m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-row">
            <div>
                <A href="/">
                    <button
                        class="bg-slate-600 rounded-r-lg size-[30px] pl-1"
                    >
                        <img src="public/icon-home-blue.svg"/>
                    </button>
                </A>
            </div>
            <div class="flex flex-col px-2 pb-2 w-full">
                <div class="py-2 flex flex-col items-center text-xl bg-indigo-600 text-white">"SHORTCUTS"</div>
                <div class="flex flex-row overflow-scroll w-full">
                    <div class="flex flex-col w-fit px-1">
                        <div class=SUBTITLE_BAR>"PASS"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"ps\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pi\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pb\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Blocked\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"po\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pc\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"GOAL KICK"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"gks\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Goal Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gki\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Goal Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gko\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Goal Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"SHOT"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"son\""</span>" => Shot { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"On target\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"sof\""</span>" => Shot { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Off target\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"sb\""</span>" => Shot { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Blocked\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"sg\""</span>" => Shot { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Goal\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"PENALTY KICK PASS"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkps\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkpi\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkpo\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkpc\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"PENALTY KICK"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkon\""</span>" => Shot { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"On target\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkof\""</span>" => Shot { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Off target\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkg\""</span>" => Shot { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Goal\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"DRIBBLE"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"drp\""</span>" => Dribble { name: "<span class=TEXT_HIGHLIGHT>"\"Dribble\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"drs\""</span>" => Dribble { name: "<span class=TEXT_HIGHLIGHT>"\"Dribble\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"drlb\""</span>" => Dribble { name: "<span class=TEXT_HIGHLIGHT>"\"Dribble\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Lost ball\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"drfw\""</span>" => Dribble { name: "<span class=TEXT_HIGHLIGHT>"\"Dribble\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Foul Won\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"CROSSING"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"crs\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Cross\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"cri\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Cross\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"crc\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Cross\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"crb\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Cross\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Blocked\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"cro\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Cross\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"THROW IN"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"tis\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"tii\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"tic\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"tio\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"CORNER KICK"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"cks\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Corner Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"cki\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Corner Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"ckc\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Corner Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"cko\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Corner Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"ckg\""</span>" => Shot { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Corner Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Goal\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"FREE KICK SHOT"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkon\""</span>" => Shot { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"On Target\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkof\""</span>" => Shot { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Off Target\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkb\""</span>" => Shot { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Blocked\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"FREE KICK PASS"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkps\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkpi\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkpo\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkpc\""</span>" => Pass { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                    </div>
                    <div class="flex flex-col w-fit px-1 ml-4">
                        <div class=SUBTITLE_BAR>"RECOVERY & LOST BALL"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"r\""</span>" => Recovery { name: "<span class=TEXT_HIGHLIGHT>"\"Recovery\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"lb\""</span>" => LostBall { name: "<span class=TEXT_HIGHLIGHT>"\"Lost Ball\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"FOUL"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"fl\""</span>" => Foul { name: "<span class=TEXT_HIGHLIGHT>"\"Fouled\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fw\""</span>" => Foul { name: "<span class=TEXT_HIGHLIGHT>"\"Foul Won\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"TACKLE"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"tw\""</span>" => Tackle { name: "<span class=TEXT_HIGHLIGHT>"\"Tackle\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Won\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"tl\""</span>" => Tackle { name: "<span class=TEXT_HIGHLIGHT>"\"Tackle\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Lost\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"DUEL"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"daw\""</span>" => Duel { name: "<span class=TEXT_HIGHLIGHT>"\"Duel\""</span>", duel_type: "<span class=TEXT_HIGHLIGHT>"\"Aerial\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Won\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"dal\""</span>" => Duel { name: "<span class=TEXT_HIGHLIGHT>"\"Duel\""</span>", duel_type: "<span class=TEXT_HIGHLIGHT>"\"Aerial\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Lose\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"dgw\""</span>" => Duel { name: "<span class=TEXT_HIGHLIGHT>"\"Duel\""</span>", duel_type: "<span class=TEXT_HIGHLIGHT>"\"Ground\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Won\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"dgl\""</span>" => Duel { name: "<span class=TEXT_HIGHLIGHT>"\"Duel\""</span>", duel_type: "<span class=TEXT_HIGHLIGHT>"\"Ground\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Lose\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"INTERCEPT"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"iop\""</span>" => Intercept { name: "<span class=TEXT_HIGHLIGHT>"\"Intercept\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"ifk\""</span>" => Intercept { name: "<span class=TEXT_HIGHLIGHT>"\"Intercept\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Freekick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"ick\""</span>" => Intercept { name: "<span class=TEXT_HIGHLIGHT>"\"Intercept\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Cornerkick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"iti\""</span>" => Intercept { name: "<span class=TEXT_HIGHLIGHT>"\"Intercept\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"CLEARANCE"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"clrop\""</span>" => Clearance { name: "<span class=TEXT_HIGHLIGHT>"\"Clearance\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"clrfk\""</span>" => Clearance { name: "<span class=TEXT_HIGHLIGHT>"\"Clearance\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Freekick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"clrck\""</span>" => Clearance { name: "<span class=TEXT_HIGHLIGHT>"\"Clearance\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Cornerkick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"clrti\""</span>" => Clearance { name: "<span class=TEXT_HIGHLIGHT>"\"Clearance\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"BLOCK"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"bs\""</span>" => Block { name: "<span class=TEXT_HIGHLIGHT>"\"Block\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"bp\""</span>" => Block { name: "<span class=TEXT_HIGHLIGHT>"\"Block\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"bcr\""</span>" => Block { name: "<span class=TEXT_HIGHLIGHT>"\"Block\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Crossing\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"PRESSURE"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"prop\""</span>" => Pressure { name: "<span class=TEXT_HIGHLIGHT>"\"Pressure\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"GOALKEEPER SAVE"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"svs\""</span>" => Save { name: "<span class=TEXT_HIGHLIGHT>"\"Save\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"svp\""</span>" => Save { name: "<span class=TEXT_HIGHLIGHT>"\"Save\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Penalty\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"svfk\""</span>" => Save { name: "<span class=TEXT_HIGHLIGHT>"\"Save\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Freekick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"svck\""</span>" => Save { name: "<span class=TEXT_HIGHLIGHT>"\"Save\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Cornerkick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"svti\""</span>" => Save { name: "<span class=TEXT_HIGHLIGHT>"\"Save\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"GOALKEEPEER CATCH"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"gccr\""</span>" => Catch { name: "<span class=TEXT_HIGHLIGHT>"\"Catch\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Crossing\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gcp\""</span>" => Catch { name: "<span class=TEXT_HIGHLIGHT>"\"Catch\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gcfk\""</span>" => Catch { name: "<span class=TEXT_HIGHLIGHT>"\"Catch\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Freekick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gcck\""</span>" => Catch { name: "<span class=TEXT_HIGHLIGHT>"\"Catch\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Cornerkick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gcti\""</span>" => Catch { name: "<span class=TEXT_HIGHLIGHT>"\"Catch\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>" },"</p>
                    </div>
                    <div class="flex flex-col 2-fit px-1 ml-4">
                        <div class=SUBTITLE_BAR>"TIME MARKER"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"kofh\""</span>" => TimeMarker { name: "<span class=TEXT_HIGHLIGHT>"\"Kick Off First Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"kosh\""</span>" => TimeMarker { name: "<span class=TEXT_HIGHLIGHT>"\"Kick Off Second Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"koefh\""</span>" => TimeMarker { name: "<span class=TEXT_HIGHLIGHT>"\"Kick Off Extra Time First Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"koesh\""</span>" => TimeMarker { name: "<span class=TEXT_HIGHLIGHT>"\"Kick Off Extra Time Second Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"eofh\""</span>" => TimeMarker { name: "<span class=TEXT_HIGHLIGHT>"\"End of First Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"eosh\""</span>" => TimeMarker { name: "<span class=TEXT_HIGHLIGHT>"\"End of Second Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"eoefh\""</span>" => TimeMarker { name: "<span class=TEXT_HIGHLIGHT>"\"End of Extra Time First Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"eoesh\""</span>" => TimeMarker { name: "<span class=TEXT_HIGHLIGHT>"\"End of Extra Time Second Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"eom\""</span>" => TimeMarker { name: "<span class=TEXT_HIGHLIGHT>"\"End of Match\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"PLAY"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"startgk\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"GK\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startrfb\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"RFB\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startlfb\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"LFB\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startcb\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"CB\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startmf\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"MF\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startrwg\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"RWG\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startlwg\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"LWG\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startcf\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"CF\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"CHANGE POSITION"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"changegk\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"GK\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changerfb\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"RFB\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changelfb\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"LFB\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changecb\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"CB\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changemf\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"MF\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changerwg\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"RWG\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changelwf\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"LWG\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changecf\""</span>" => Play { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"CF\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"SUBSTITUTION"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"subs\""</span>" => Subs { name: "<span class=TEXT_HIGHLIGHT>"\"Subs\""</span>", team: team_args, subs_in: player_args },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"OTHER"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"yc\""</span>" => Other { name: "<span class=TEXT_HIGHLIGHT>"\"Yellow Card\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"syc\""</span>" => Other { name: "<span class=TEXT_HIGHLIGHT>"\"Second Yellow Card\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"rc\""</span>" => Other { name: "<span class=TEXT_HIGHLIGHT>"\"Red Card\""</span>" },"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
