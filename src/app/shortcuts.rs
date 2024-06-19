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
                        <img src="public/home.svg" width="20" height="20"/>
                    </button>
                </A>
            </div>
            <div class="flex flex-col px-2 pb-2 w-full">
                <div class="py-2 flex flex-col items-center text-xl bg-indigo-600 text-white">"SHORTCUTS"</div>
                <div class="bg-green-300 w-fit rounded-lg px-2 py-1 mt-1">"GUIDES"</div>
                <div class="text-wrap pl-2 pr-10 mb-1">
                    <p>
                        "Command shortcuts are separated by \"/\".
                        for example: 3h/pi/9a, will be translated into: [PLAYER] player number [3] from team [h]ome / [EVENT] [p]ass [i]ntercepted / [OUTCOME] intercepted by player number [9] from team [a]way.
                        Events which are categorized as pass, need 3 commands to register: the [PLAYER], the [EVENT], and the [OUTCOME]. The [PLAYER] is the guy doing the action. The [EVENT] is the action.
                        The [OUTCOME] is the player at the end of an action, for example a player receiving or intercepting the pass, etc. In case you forget to register the [OUTCOME],
                        it would still be okay and the event would still be registered. It's just that later on you will need to do more effort when analyzing the data. Substitution also requires you
                        to register the [OUTCOME]."
                    </p>
                </div>
                <div class="flex flex-row overflow-scroll w-full">
                    <div class="flex flex-col w-fit px-1">
                        <div class=SUBTITLE_BAR>"PASS"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"ps\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pi\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Blocked\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"po\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pc\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pp\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Punched\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"GOAL KICK"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"gks\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Goal Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gki\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Goal Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gko\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Goal Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gko\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Goal Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gkp\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Goal Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Punched\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"SHOT"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"son\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"On target\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"sof\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Off target\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"sb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Blocked\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"sg\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Goal\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"PENALTY KICK PASS"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkps\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkpi\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkpo\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkpc\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"PENALTY KICK"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkon\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"On target\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkof\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Off target\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"pkg\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Penalty Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Goal\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"DRIBBLE"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"drp\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Dribble\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"drs\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Dribble\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"drlb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Dribble\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Lost ball\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"drfw\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Dribble\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Foul Won\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"CROSSING"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"crs\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Cross\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"cri\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Cross\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"crc\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Cross\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"crb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Cross\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Blocked\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"cro\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Cross\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"crp\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Cross\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Punched\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"THROW IN"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"tis\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"tii\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"tic\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"tio\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"tip\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Punched\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"CORNER KICK"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"cks\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Corner Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"cki\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Corner Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"ckc\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Corner Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"cko\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Corner Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"ckp\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Corner Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Punched\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"ckg\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Corner Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Goal\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"FREE KICK SHOT"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkon\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"On Target\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkof\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Off Target\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>", shot_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Blocked\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"FREE KICK PASS"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkps\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Success\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkpi\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Intercepted\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkpo\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Out of Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkpc\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Catched\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fkpp\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>", pass_type: "<span class=TEXT_HIGHLIGHT>"\"Free Kick\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Punched\""</span>" },"</p>
                    </div>
                    <div class="flex flex-col w-fit px-1 ml-9">
                        <div class=SUBTITLE_BAR>"RECOVERY & LOST BALL"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"r\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Recovery\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"lb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Lost Ball\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"FOUL"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"fl\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Fouled\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"fw\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Foul Won\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"TACKLE"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"tw\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Tackle\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Won\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"tl\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Tackle\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Lost\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"DUEL"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"daw\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Duel\""</span>", duel_type: "<span class=TEXT_HIGHLIGHT>"\"Aerial\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Won\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"dal\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Duel\""</span>", duel_type: "<span class=TEXT_HIGHLIGHT>"\"Aerial\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Lose\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"dgw\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Duel\""</span>", duel_type: "<span class=TEXT_HIGHLIGHT>"\"Ground\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Won\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"dgl\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Duel\""</span>", duel_type: "<span class=TEXT_HIGHLIGHT>"\"Ground\""</span>", outcome: "<span class=TEXT_HIGHLIGHT>"\"Lose\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"INTERCEPT"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"iop\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Intercept\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"ifk\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Intercept\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Freekick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"ick\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Intercept\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Cornerkick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"iti\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Intercept\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"CLEARANCE"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"clrop\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Clearance\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Open Play\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"clrfk\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Clearance\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Freekick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"clrck\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Clearance\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Cornerkick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"clrti\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Clearance\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"BLOCK"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"bs\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Block\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"bp\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Block\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"bcr\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Block\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Crossing\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"PRESSURE"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"pr\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Pressure\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"GOALKEEPER SAVE"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"svs\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Save\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Shot\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"svpk\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Save\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Penalty\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"svfk\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Save\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Freekick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"svck\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Save\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Cornerkick\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"GOALKEEPEER CATCH"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"gccr\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Catch\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Crossing\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gcp\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Catch\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Pass\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gcfk\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Catch\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Freekick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gcck\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Catch\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Cornerkick\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"gcti\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Catch\""</span>", event_source: "<span class=TEXT_HIGHLIGHT>"\"Throw In\""</span>" },"</p>
                    </div>
                    <div class="flex flex-col 2-fit px-1 ml-9">
                        <div class=SUBTITLE_BAR>"TIME MARKER"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"kofh\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Kick Off First Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"kosh\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Kick Off Second Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"koefh\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Kick Off Extra Time First Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"koesh\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Kick Off Extra Time Second Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"eofh\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"End of First Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"eosh\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"End of Second Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"eoefh\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"End of Extra Time First Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"eoesh\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"End of Extra Time Second Half\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"eom\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"End of Match\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"PLAY"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"startgk\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"GK\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startrfb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"RFB\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startlfb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"LFB\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startcb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"CB\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startmf\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"MF\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startrwg\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"RWG\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startlwg\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"LWG\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"startcf\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Start\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"CF\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"CHANGE POSITION"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"changegk\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"GK\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changerfb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"RFB\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changelfb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"LFB\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changecb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"CB\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changemf\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"MF\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changerwg\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"RWG\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changelwf\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"LWG\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"changecf\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Change Position\""</span>", position: "<span class=TEXT_HIGHLIGHT>"\"CF\""</span>" },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"SUBSTITUTION"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"sigk\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Subs In\""</span>", play_position: "<span class=TEXT_HIGHLIGHT>"\"GK\""</span>" team: team_args, subs_in: player_args },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"sirfb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Subs In\""</span>", play_position: "<span class=TEXT_HIGHLIGHT>"\"RFB\""</span>" team: team_args, subs_in: player_args },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"silfb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Subs In\""</span>", play_position: "<span class=TEXT_HIGHLIGHT>"\"LFB\""</span>" team: team_args, subs_in: player_args },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"sicb\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Subs In\""</span>", play_position: "<span class=TEXT_HIGHLIGHT>"\"CB\""</span>" team: team_args, subs_in: player_args },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"simf\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Subs In\""</span>", play_position: "<span class=TEXT_HIGHLIGHT>"\"MF\""</span>" team: team_args, subs_in: player_args },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"sirwg\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Subs In\""</span>", play_position: "<span class=TEXT_HIGHLIGHT>"\"RWG\""</span>" team: team_args, subs_in: player_args },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"silwg\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Subs In\""</span>", play_position: "<span class=TEXT_HIGHLIGHT>"\"LWG\""</span>" team: team_args, subs_in: player_args },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"sicf\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Subs In\""</span>", play_position: "<span class=TEXT_HIGHLIGHT>"\"CF\""</span>" team: team_args, subs_in: player_args },"</p>
                        <br/>
                        <div class=SUBTITLE_BAR>"OTHER"</div>
                        <p><span class=TEXT_HIGHLIGHT>"\"yc\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Yellow Card\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"syc\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Second Yellow Card\""</span>" },"</p>
                        <p><span class=TEXT_HIGHLIGHT>"\"rc\""</span>" => { name: "<span class=TEXT_HIGHLIGHT>"\"Red Card\""</span>" },"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
