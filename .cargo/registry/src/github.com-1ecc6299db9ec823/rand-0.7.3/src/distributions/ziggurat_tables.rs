// Copyright 2018 Developers of the Rand project.
// Copyright 2013 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Tables for distributions which are sampled using the ziggurat
// algorithm. Autogenerated by `ziggurat_tables.py`.

pub type ZigTable = &'static [f64; 257];
pub const ZIG_NORM_R: f64 = 3.654152885361008796;
#[rustfmt::skip]
pub static ZIG_NORM_X: [f64; 257] =
    [3.910757959537090045, 3.654152885361008796, 3.449278298560964462, 3.320244733839166074,
     3.224575052047029100, 3.147889289517149969, 3.083526132001233044, 3.027837791768635434,
     2.978603279880844834, 2.934366867207854224, 2.894121053612348060, 2.857138730872132548,
     2.822877396825325125, 2.790921174000785765, 2.760944005278822555, 2.732685359042827056,
     2.705933656121858100, 2.680514643284522158, 2.656283037575502437, 2.633116393630324570,
     2.610910518487548515, 2.589575986706995181, 2.569035452680536569, 2.549221550323460761,
     2.530075232158516929, 2.511544441625342294, 2.493583041269680667, 2.476149939669143318,
     2.459208374333311298, 2.442725318198956774, 2.426670984935725972, 2.411018413899685520,
     2.395743119780480601, 2.380822795170626005, 2.366237056715818632, 2.351967227377659952,
     2.337996148795031370, 2.324308018869623016, 2.310888250599850036, 2.297723348901329565,
     2.284800802722946056, 2.272108990226823888, 2.259637095172217780, 2.247375032945807760,
     2.235313384928327984, 2.223443340090905718, 2.211756642882544366, 2.200245546609647995,
     2.188902771624720689, 2.177721467738641614, 2.166695180352645966, 2.155817819875063268,
     2.145083634046203613, 2.134487182844320152, 2.124023315687815661, 2.113687150684933957,
     2.103474055713146829, 2.093379631137050279, 2.083399693996551783, 2.073530263516978778,
     2.063767547809956415, 2.054107931648864849, 2.044547965215732788, 2.035084353727808715,
     2.025713947862032960, 2.016433734904371722, 2.007240830558684852, 1.998132471356564244,
     1.989106007615571325, 1.980158896898598364, 1.971288697931769640, 1.962493064942461896,
     1.953769742382734043, 1.945116560006753925, 1.936531428273758904, 1.928012334050718257,
     1.919557336591228847, 1.911164563769282232, 1.902832208548446369, 1.894558525668710081,
     1.886341828534776388, 1.878180486290977669, 1.870072921069236838, 1.862017605397632281,
     1.854013059758148119, 1.846057850283119750, 1.838150586580728607, 1.830289919680666566,
     1.822474540091783224, 1.814703175964167636, 1.806974591348693426, 1.799287584547580199,
     1.791640986550010028, 1.784033659547276329, 1.776464495522344977, 1.768932414909077933,
     1.761436365316706665, 1.753975320315455111, 1.746548278279492994, 1.739154261283669012,
     1.731792314050707216, 1.724461502945775715, 1.717160915015540690, 1.709889657069006086,
     1.702646854797613907, 1.695431651932238548, 1.688243209434858727, 1.681080704722823338,
     1.673943330923760353, 1.666830296159286684, 1.659740822855789499, 1.652674147080648526,
     1.645629517902360339, 1.638606196773111146, 1.631603456932422036, 1.624620582830568427,
     1.617656869570534228, 1.610711622367333673, 1.603784156023583041, 1.596873794420261339,
     1.589979870021648534, 1.583101723393471438, 1.576238702733332886, 1.569390163412534456,
     1.562555467528439657, 1.555733983466554893, 1.548925085471535512, 1.542128153226347553,
     1.535342571438843118, 1.528567729435024614, 1.521803020758293101, 1.515047842773992404,
     1.508301596278571965, 1.501563685112706548, 1.494833515777718391, 1.488110497054654369,
     1.481394039625375747, 1.474683555695025516, 1.467978458615230908, 1.461278162507407830,
     1.454582081885523293, 1.447889631277669675, 1.441200224845798017, 1.434513276002946425,
     1.427828197027290358, 1.421144398672323117, 1.414461289772464658, 1.407778276843371534,
     1.401094763676202559, 1.394410150925071257, 1.387723835686884621, 1.381035211072741964,
     1.374343665770030531, 1.367648583594317957, 1.360949343030101844, 1.354245316759430606,
     1.347535871177359290, 1.340820365893152122, 1.334098153216083604, 1.327368577624624679,
     1.320630975217730096, 1.313884673146868964, 1.307128989027353860, 1.300363230327433728,
     1.293586693733517645, 1.286798664489786415, 1.279998415710333237, 1.273185207661843732,
     1.266358287014688333, 1.259516886060144225, 1.252660221891297887, 1.245787495544997903,
     1.238897891102027415, 1.231990574742445110, 1.225064693752808020, 1.218119375481726552,
     1.211153726239911244, 1.204166830140560140, 1.197157747875585931, 1.190125515422801650,
     1.183069142678760732, 1.175987612011489825, 1.168879876726833800, 1.161744859441574240,
     1.154581450355851802, 1.147388505416733873, 1.140164844363995789, 1.132909248648336975,
     1.125620459211294389, 1.118297174115062909, 1.110938046009249502, 1.103541679420268151,
     1.096106627847603487, 1.088631390649514197, 1.081114409698889389, 1.073554065787871714,
     1.065948674757506653, 1.058296483326006454, 1.050595664586207123, 1.042844313139370538,
     1.035040439828605274, 1.027181966030751292, 1.019266717460529215, 1.011292417434978441,
     1.003256679539591412, 0.995156999629943084, 0.986990747093846266, 0.978755155288937750,
     0.970447311058864615, 0.962064143217605250, 0.953602409875572654, 0.945058684462571130,
     0.936429340280896860, 0.927710533396234771, 0.918898183643734989, 0.909987953490768997,
     0.900975224455174528, 0.891855070726792376, 0.882622229578910122, 0.873271068082494550,
     0.863795545546826915, 0.854189171001560554, 0.844444954902423661, 0.834555354079518752,
     0.824512208745288633, 0.814306670128064347, 0.803929116982664893, 0.793369058833152785,
     0.782615023299588763, 0.771654424216739354, 0.760473406422083165, 0.749056662009581653,
     0.737387211425838629, 0.725446140901303549, 0.713212285182022732, 0.700661841097584448,
     0.687767892786257717, 0.674499822827436479, 0.660822574234205984, 0.646695714884388928,
     0.632072236375024632, 0.616896989996235545, 0.601104617743940417, 0.584616766093722262,
     0.567338257040473026, 0.549151702313026790, 0.529909720646495108, 0.509423329585933393,
     0.487443966121754335, 0.463634336771763245, 0.437518402186662658, 0.408389134588000746,
     0.375121332850465727, 0.335737519180459465, 0.286174591747260509, 0.215241895913273806,
     0.000000000000000000];
#[rustfmt::skip]
pub static ZIG_NORM_F: [f64; 257] =
    [0.000477467764586655, 0.001260285930498598, 0.002609072746106363, 0.004037972593371872,
     0.005522403299264754, 0.007050875471392110, 0.008616582769422917, 0.010214971439731100,
     0.011842757857943104, 0.013497450601780807, 0.015177088307982072, 0.016880083152595839,
     0.018605121275783350, 0.020351096230109354, 0.022117062707379922, 0.023902203305873237,
     0.025705804008632656, 0.027527235669693315, 0.029365939758230111, 0.031221417192023690,
     0.033093219458688698, 0.034980941461833073, 0.036884215688691151, 0.038802707404656918,
     0.040736110656078753, 0.042684144916619378, 0.044646552251446536, 0.046623094902089664,
     0.048613553216035145, 0.050617723861121788, 0.052635418276973649, 0.054666461325077916,
     0.056710690106399467, 0.058767952921137984, 0.060838108349751806, 0.062921024437977854,
     0.065016577971470438, 0.067124653828023989, 0.069245144397250269, 0.071377949059141965,
     0.073522973714240991, 0.075680130359194964, 0.077849336702372207, 0.080030515814947509,
     0.082223595813495684, 0.084428509570654661, 0.086645194450867782, 0.088873592068594229,
     0.091113648066700734, 0.093365311913026619, 0.095628536713353335, 0.097903279039215627,
     0.100189498769172020, 0.102487158942306270, 0.104796225622867056, 0.107116667775072880,
     0.109448457147210021, 0.111791568164245583, 0.114145977828255210, 0.116511665626037014,
     0.118888613443345698, 0.121276805485235437, 0.123676228202051403, 0.126086870220650349,
     0.128508722280473636, 0.130941777174128166, 0.133386029692162844, 0.135841476571757352,
     0.138308116449064322, 0.140785949814968309, 0.143274978974047118, 0.145775208006537926,
     0.148286642733128721, 0.150809290682410169, 0.153343161060837674, 0.155888264725064563,
     0.158444614156520225, 0.161012223438117663, 0.163591108232982951, 0.166181285765110071,
     0.168782774801850333, 0.171395595638155623, 0.174019770082499359, 0.176655321444406654,
     0.179302274523530397, 0.181960655600216487, 0.184630492427504539, 0.187311814224516926,
     0.190004651671193070, 0.192709036904328807, 0.195425003514885592, 0.198152586546538112,
     0.200891822495431333, 0.203642749311121501, 0.206405406398679298, 0.209179834621935651,
     0.211966076307852941, 0.214764175252008499, 0.217574176725178370, 0.220396127481011589,
     0.223230075764789593, 0.226076071323264877, 0.228934165415577484, 0.231804410825248525,
     0.234686861873252689, 0.237581574432173676, 0.240488605941449107, 0.243408015423711988,
     0.246339863502238771, 0.249284212419516704, 0.252241126056943765, 0.255210669955677150,
     0.258192911338648023, 0.261187919133763713, 0.264195763998317568, 0.267216518344631837,
     0.270250256366959984, 0.273297054069675804, 0.276356989296781264, 0.279430141762765316,
     0.282516593084849388, 0.285616426816658109, 0.288729728483353931, 0.291856585618280984,
     0.294997087801162572, 0.298151326697901342, 0.301319396102034120, 0.304501391977896274,
     0.307697412505553769, 0.310907558127563710, 0.314131931597630143, 0.317370638031222396,
     0.320623784958230129, 0.323891482377732021, 0.327173842814958593, 0.330470981380537099,
     0.333783015832108509, 0.337110066638412809, 0.340452257045945450, 0.343809713148291340,
     0.347182563958251478, 0.350570941482881204, 0.353974980801569250, 0.357394820147290515,
     0.360830600991175754, 0.364282468130549597, 0.367750569780596226, 0.371235057669821344,
     0.374736087139491414, 0.378253817247238111, 0.381788410875031348, 0.385340034841733958,
     0.388908860020464597, 0.392495061461010764, 0.396098818517547080, 0.399720314981931668,
     0.403359739222868885, 0.407017284331247953, 0.410693148271983222, 0.414387534042706784,
     0.418100649839684591, 0.421832709231353298, 0.425583931339900579, 0.429354541031341519,
     0.433144769114574058, 0.436954852549929273, 0.440785034667769915, 0.444635565397727750,
     0.448506701509214067, 0.452398706863882505, 0.456311852680773566, 0.460246417814923481,
     0.464202689050278838, 0.468180961407822172, 0.472181538469883255, 0.476204732721683788,
     0.480250865911249714, 0.484320269428911598, 0.488413284707712059, 0.492530263646148658,
     0.496671569054796314, 0.500837575128482149, 0.505028667945828791, 0.509245245998136142,
     0.513487720749743026, 0.517756517232200619, 0.522052074674794864, 0.526374847174186700,
     0.530725304406193921, 0.535103932383019565, 0.539511234259544614, 0.543947731192649941,
     0.548413963257921133, 0.552910490428519918, 0.557437893621486324, 0.561996775817277916,
     0.566587763258951771, 0.571211506738074970, 0.575868682975210544, 0.580559996103683473,
     0.585286179266300333, 0.590047996335791969, 0.594846243770991268, 0.599681752622167719,
     0.604555390700549533, 0.609468064928895381, 0.614420723892076803, 0.619414360609039205,
     0.624450015550274240, 0.629528779928128279, 0.634651799290960050, 0.639820277456438991,
     0.645035480824251883, 0.650298743114294586, 0.655611470583224665, 0.660975147780241357,
     0.666391343912380640, 0.671861719900766374, 0.677388036222513090, 0.682972161648791376,
     0.688616083008527058, 0.694321916130032579, 0.700091918140490099, 0.705928501336797409,
     0.711834248882358467, 0.717811932634901395, 0.723864533472881599, 0.729995264565802437,
     0.736207598131266683, 0.742505296344636245, 0.748892447223726720, 0.755373506511754500,
     0.761953346841546475, 0.768637315803334831, 0.775431304986138326, 0.782341832659861902,
     0.789376143571198563, 0.796542330428254619, 0.803849483176389490, 0.811307874318219935,
     0.818929191609414797, 0.826726833952094231, 0.834716292992930375, 0.842915653118441077,
     0.851346258465123684, 0.860033621203008636, 0.869008688043793165, 0.878309655816146839,
     0.887984660763399880, 0.898095921906304051, 0.908726440060562912, 0.919991505048360247,
     0.932060075968990209, 0.945198953453078028, 0.959879091812415930, 0.977101701282731328,
     1.000000000000000000];
pub const ZIG_EXP_R: f64 = 7.697117470131050077;
#[rustfmt::skip]
pub static ZIG_EXP_X: [f64; 257] =
    [8.697117470131052741, 7.697117470131050077, 6.941033629377212577, 6.478378493832569696,
     6.144164665772472667, 5.882144315795399869, 5.666410167454033697, 5.482890627526062488,
     5.323090505754398016, 5.181487281301500047, 5.054288489981304089, 4.938777085901250530,
     4.832939741025112035, 4.735242996601741083, 4.644491885420085175, 4.559737061707351380,
     4.480211746528421912, 4.405287693473573185, 4.334443680317273007, 4.267242480277365857,
     4.203313713735184365, 4.142340865664051464, 4.084051310408297830, 4.028208544647936762,
     3.974606066673788796, 3.923062500135489739, 3.873417670399509127, 3.825529418522336744,
     3.779270992411667862, 3.734528894039797375, 3.691201090237418825, 3.649195515760853770,
     3.608428813128909507, 3.568825265648337020, 3.530315889129343354, 3.492837654774059608,
     3.456332821132760191, 3.420748357251119920, 3.386035442460300970, 3.352149030900109405,
     3.319047470970748037, 3.286692171599068679, 3.255047308570449882, 3.224079565286264160,
     3.193757903212240290, 3.164053358025972873, 3.134938858084440394, 3.106389062339824481,
     3.078380215254090224, 3.050890016615455114, 3.023897504455676621, 2.997382949516130601,
     2.971327759921089662, 2.945714394895045718, 2.920526286512740821, 2.895747768600141825,
     2.871364012015536371, 2.847360965635188812, 2.823725302450035279, 2.800444370250737780,
     2.777506146439756574, 2.754899196562344610, 2.732612636194700073, 2.710636095867928752,
     2.688959688741803689, 2.667573980773266573, 2.646469963151809157, 2.625639026797788489,
     2.605072938740835564, 2.584763820214140750, 2.564704126316905253, 2.544886627111869970,
     2.525304390037828028, 2.505950763528594027, 2.486819361740209455, 2.467904050297364815,
     2.449198932978249754, 2.430698339264419694, 2.412396812688870629, 2.394289099921457886,
     2.376370140536140596, 2.358635057409337321, 2.341079147703034380, 2.323697874390196372,
     2.306486858283579799, 2.289441870532269441, 2.272558825553154804, 2.255833774367219213,
     2.239262898312909034, 2.222842503111036816, 2.206569013257663858, 2.190438966723220027,
     2.174449009937774679, 2.158595893043885994, 2.142876465399842001, 2.127287671317368289,
     2.111826546019042183, 2.096490211801715020, 2.081275874393225145, 2.066180819490575526,
     2.051202409468584786, 2.036338080248769611, 2.021585338318926173, 2.006941757894518563,
     1.992404978213576650, 1.977972700957360441, 1.963642687789548313, 1.949412758007184943,
     1.935280786297051359, 1.921244700591528076, 1.907302480018387536, 1.893452152939308242,
     1.879691795072211180, 1.866019527692827973, 1.852433515911175554, 1.838931967018879954,
     1.825513128903519799, 1.812175288526390649, 1.798916770460290859, 1.785735935484126014,
     1.772631179231305643, 1.759600930889074766, 1.746643651946074405, 1.733757834985571566,
     1.720942002521935299, 1.708194705878057773, 1.695514524101537912, 1.682900062917553896,
     1.670349953716452118, 1.657862852574172763, 1.645437439303723659, 1.633072416535991334,
     1.620766508828257901, 1.608518461798858379, 1.596327041286483395, 1.584191032532688892,
     1.572109239386229707, 1.560080483527888084, 1.548103603714513499, 1.536177455041032092,
     1.524300908219226258, 1.512472848872117082, 1.500692176842816750, 1.488957805516746058,
     1.477268661156133867, 1.465623682245745352, 1.454021818848793446, 1.442462031972012504,
     1.430943292938879674, 1.419464582769983219, 1.408024891569535697, 1.396623217917042137,
     1.385258568263121992, 1.373929956328490576, 1.362636402505086775, 1.351376933258335189,
     1.340150580529504643, 1.328956381137116560, 1.317793376176324749, 1.306660610415174117,
     1.295557131686601027, 1.284481990275012642, 1.273434238296241139, 1.262412929069615330,
     1.251417116480852521, 1.240445854334406572, 1.229498195693849105, 1.218573192208790124,
     1.207669893426761121, 1.196787346088403092, 1.185924593404202199, 1.175080674310911677,
     1.164254622705678921, 1.153445466655774743, 1.142652227581672841, 1.131873919411078511,
     1.121109547701330200, 1.110358108727411031, 1.099618588532597308, 1.088889961938546813,
     1.078171191511372307, 1.067461226479967662, 1.056759001602551429, 1.046063435977044209,
     1.035373431790528542, 1.024687873002617211, 1.014005623957096480, 1.003325527915696735,
     0.992646405507275897, 0.981967053085062602, 0.971286240983903260, 0.960602711668666509,
     0.949915177764075969, 0.939222319955262286, 0.928522784747210395, 0.917815182070044311,
     0.907098082715690257, 0.896370015589889935, 0.885629464761751528, 0.874874866291025066,
     0.864104604811004484, 0.853317009842373353, 0.842510351810368485, 0.831682837734273206,
     0.820832606554411814, 0.809957724057418282, 0.799056177355487174, 0.788125868869492430,
     0.777164609759129710, 0.766170112735434672, 0.755139984181982249, 0.744071715500508102,
     0.732962673584365398, 0.721810090308756203, 0.710611050909655040, 0.699362481103231959,
     0.688061132773747808, 0.676703568029522584, 0.665286141392677943, 0.653804979847664947,
     0.642255960424536365, 0.630634684933490286, 0.618936451394876075, 0.607156221620300030,
     0.595288584291502887, 0.583327712748769489, 0.571267316532588332, 0.559100585511540626,
     0.546820125163310577, 0.534417881237165604, 0.521885051592135052, 0.509211982443654398,
     0.496388045518671162, 0.483401491653461857, 0.470239275082169006, 0.456886840931420235,
     0.443327866073552401, 0.429543940225410703, 0.415514169600356364, 0.401214678896277765,
     0.386617977941119573, 0.371692145329917234, 0.356399760258393816, 0.340696481064849122,
     0.324529117016909452, 0.307832954674932158, 0.290527955491230394, 0.272513185478464703,
     0.253658363385912022, 0.233790483059674731, 0.212671510630966620, 0.189958689622431842,
     0.165127622564187282, 0.137304980940012589, 0.104838507565818778, 0.063852163815001570,
     0.000000000000000000];
#[rustfmt::skip]
pub static ZIG_EXP_F: [f64; 257] =
    [0.000167066692307963, 0.000454134353841497, 0.000967269282327174, 0.001536299780301573,
     0.002145967743718907, 0.002788798793574076, 0.003460264777836904, 0.004157295120833797,
     0.004877655983542396, 0.005619642207205489, 0.006381905937319183, 0.007163353183634991,
     0.007963077438017043, 0.008780314985808977, 0.009614413642502212, 0.010464810181029981,
     0.011331013597834600, 0.012212592426255378, 0.013109164931254991, 0.014020391403181943,
     0.014945968011691148, 0.015885621839973156, 0.016839106826039941, 0.017806200410911355,
     0.018786700744696024, 0.019780424338009740, 0.020787204072578114, 0.021806887504283581,
     0.022839335406385240, 0.023884420511558174, 0.024942026419731787, 0.026012046645134221,
     0.027094383780955803, 0.028188948763978646, 0.029295660224637411, 0.030414443910466622,
     0.031545232172893622, 0.032687963508959555, 0.033842582150874358, 0.035009037697397431,
     0.036187284781931443, 0.037377282772959382, 0.038578995503074871, 0.039792391023374139,
     0.041017441380414840, 0.042254122413316254, 0.043502413568888197, 0.044762297732943289,
     0.046033761076175184, 0.047316792913181561, 0.048611385573379504, 0.049917534282706379,
     0.051235237055126281, 0.052564494593071685, 0.053905310196046080, 0.055257689676697030,
     0.056621641283742870, 0.057997175631200659, 0.059384305633420280, 0.060783046445479660,
     0.062193415408541036, 0.063615431999807376, 0.065049117786753805, 0.066494496385339816,
     0.067951593421936643, 0.069420436498728783, 0.070901055162371843, 0.072393480875708752,
     0.073897746992364746, 0.075413888734058410, 0.076941943170480517, 0.078481949201606435,
     0.080033947542319905, 0.081597980709237419, 0.083174093009632397, 0.084762330532368146,
     0.086362741140756927, 0.087975374467270231, 0.089600281910032886, 0.091237516631040197,
     0.092887133556043569, 0.094549189376055873, 0.096223742550432825, 0.097910853311492213,
     0.099610583670637132, 0.101322997425953631, 0.103048160171257702, 0.104786139306570145,
     0.106537004050001632, 0.108300825451033755, 0.110077676405185357, 0.111867631670056283,
     0.113670767882744286, 0.115487163578633506, 0.117316899211555525, 0.119160057175327641,
     0.121016721826674792, 0.122886979509545108, 0.124770918580830933, 0.126668629437510671,
     0.128580204545228199, 0.130505738468330773, 0.132445327901387494, 0.134399071702213602,
     0.136367070926428829, 0.138349428863580176, 0.140346251074862399, 0.142357645432472146,
     0.144383722160634720, 0.146424593878344889, 0.148480375643866735, 0.150551185001039839,
     0.152637142027442801, 0.154738369384468027, 0.156854992369365148, 0.158987138969314129,
     0.161134939917591952, 0.163298528751901734, 0.165478041874935922, 0.167673618617250081,
     0.169885401302527550, 0.172113535315319977, 0.174358169171353411, 0.176619454590494829,
     0.178897546572478278, 0.181192603475496261, 0.183504787097767436, 0.185834262762197083,
     0.188181199404254262, 0.190545769663195363, 0.192928149976771296, 0.195328520679563189,
     0.197747066105098818, 0.200183974691911210, 0.202639439093708962, 0.205113656293837654,
     0.207606827724221982, 0.210119159388988230, 0.212650861992978224, 0.215202151075378628,
     0.217773247148700472, 0.220364375843359439, 0.222975768058120111, 0.225607660116683956,
     0.228260293930716618, 0.230933917169627356, 0.233628783437433291, 0.236345152457059560,
     0.239083290262449094, 0.241843469398877131, 0.244625969131892024, 0.247431075665327543,
     0.250259082368862240, 0.253110290015629402, 0.255985007030415324, 0.258883549749016173,
     0.261806242689362922, 0.264753418835062149, 0.267725419932044739, 0.270722596799059967,
     0.273745309652802915, 0.276793928448517301, 0.279868833236972869, 0.282970414538780746,
     0.286099073737076826, 0.289255223489677693, 0.292439288161892630, 0.295651704281261252,
     0.298892921015581847, 0.302163400675693528, 0.305463619244590256, 0.308794066934560185,
     0.312155248774179606, 0.315547685227128949, 0.318971912844957239, 0.322428484956089223,
     0.325917972393556354, 0.329440964264136438, 0.332998068761809096, 0.336589914028677717,
     0.340217149066780189, 0.343880444704502575, 0.347580494621637148, 0.351318016437483449,
     0.355093752866787626, 0.358908472948750001, 0.362762973354817997, 0.366658079781514379,
     0.370594648435146223, 0.374573567615902381, 0.378595759409581067, 0.382662181496010056,
     0.386773829084137932, 0.390931736984797384, 0.395136981833290435, 0.399390684475231350,
     0.403694012530530555, 0.408048183152032673, 0.412454465997161457, 0.416914186433003209,
     0.421428728997616908, 0.425999541143034677, 0.430628137288459167, 0.435316103215636907,
     0.440065100842354173, 0.444876873414548846, 0.449753251162755330, 0.454696157474615836,
     0.459707615642138023, 0.464789756250426511, 0.469944825283960310, 0.475175193037377708,
     0.480483363930454543, 0.485871987341885248, 0.491343869594032867, 0.496901987241549881,
     0.502549501841348056, 0.508289776410643213, 0.514126393814748894, 0.520063177368233931,
     0.526104213983620062, 0.532253880263043655, 0.538516872002862246, 0.544898237672440056,
     0.551403416540641733, 0.558038282262587892, 0.564809192912400615, 0.571723048664826150,
     0.578787358602845359, 0.586010318477268366, 0.593400901691733762, 0.600968966365232560,
     0.608725382079622346, 0.616682180915207878, 0.624852738703666200, 0.633251994214366398,
     0.641896716427266423, 0.650805833414571433, 0.660000841079000145, 0.669506316731925177,
     0.679350572264765806, 0.689566496117078431, 0.700192655082788606, 0.711274760805076456,
     0.722867659593572465, 0.735038092431424039, 0.747868621985195658, 0.761463388849896838,
     0.775956852040116218, 0.791527636972496285, 0.808421651523009044, 0.826993296643051101,
     0.847785500623990496, 0.871704332381204705, 0.900469929925747703, 0.938143680862176477,
     1.000000000000000000];
