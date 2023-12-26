#![allow(dead_code)]

pub const KNIGHT_ATTACKS: [u64; 64] = [
    132096u64, 329728u64, 659712u64, 1319424u64, 2638848u64, 5277696u64, 10489856u64, 4202496u64, 33816580u64, 84410376u64, 168886289u64, 337772578u64, 675545156u64, 1351090312u64, 
    2685403152u64, 1075839008u64, 8657044482u64, 21609056261u64, 43234889994u64, 86469779988u64, 172939559976u64, 345879119952u64, 687463207072u64, 275414786112u64, 
    2216203387392u64, 5531918402816u64, 11068131838464u64, 22136263676928u64, 44272527353856u64, 88545054707712u64, 175990581010432u64, 70506185244672u64, 
    567348067172352u64, 1416171111120896u64, 2833441750646784u64, 5666883501293568u64, 11333767002587136u64, 22667534005174272u64, 45053588738670592u64, 
    18049583422636032u64, 145241105196122112u64, 362539804446949376u64, 725361088165576704u64, 1450722176331153408u64, 2901444352662306816u64, 5802888705324613632u64, 
    11533718717099671552u64, 4620693356194824192u64, 288234782788157440u64, 576469569871282176u64, 1224997833292120064u64, 2449995666584240128u64, 
    4899991333168480256u64, 9799982666336960512u64, 1152939783987658752u64, 2305878468463689728u64, 1128098930098176u64, 2257297371824128u64, 4796069720358912u64, 
    9592139440717824u64, 19184278881435648u64, 38368557762871296u64, 4679521487814656u64, 9077567998918656,
];

pub const PAWN_BLACK_ATTACKS: [u64; 64] = [
    0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 2u64, 5u64, 10u64, 20u64, 40u64, 80u64, 160u64, 64u64, 512u64, 1280u64, 2560u64, 5120u64, 10240u64, 20480u64, 40960u64, 16384u64, 131072u64, 327680u64, 655360u64, 1310720u64, 
    2621440u64, 5242880u64, 10485760u64, 4194304u64, 33554432u64, 83886080u64, 167772160u64, 335544320u64, 671088640u64, 1342177280u64, 2684354560u64, 1073741824u64, 8589934592,
    21474836480u64, 42949672960u64, 85899345920u64, 171798691840u64, 343597383680u64, 687194767360u64, 274877906944u64, 2199023255552u64, 5497558138880u64, 10995116277760u64, 
    21990232555520u64, 43980465111040u64, 87960930222080u64, 175921860444160u64, 70368744177664u64, 562949953421312u64, 1407374883553280u64, 2814749767106560u64, 5629499534213120u64, 
    11258999068426240u64, 22517998136852480u64, 45035996273704960u64, 18014398509481984,
];

pub const PAWN_WHITE_ATTACKS: [u64; 64] = [
    512u64, 1280u64, 2560u64, 5120u64, 10240u64, 20480u64, 40960u64, 16384u64, 131072u64, 327680u64, 655360u64, 1310720u64, 2621440u64, 5242880u64, 10485760u64, 4194304u64, 33554432u64, 83886080u64, 
    167772160u64, 335544320u64, 671088640u64, 1342177280u64, 2684354560u64, 1073741824u64, 8589934592u64, 21474836480u64, 42949672960u64, 85899345920u64, 171798691840u64, 343597383680u64, 
    687194767360u64, 274877906944u64, 2199023255552u64, 5497558138880u64, 10995116277760u64, 21990232555520u64, 43980465111040u64, 87960930222080u64, 175921860444160u64, 
    70368744177664u64, 562949953421312u64, 1407374883553280u64, 2814749767106560u64, 5629499534213120u64, 11258999068426240u64, 22517998136852480u64, 45035996273704960u64, 
    18014398509481984u64, 144115188075855872u64, 360287970189639680u64, 720575940379279360u64, 1441151880758558720u64, 2882303761517117440u64, 5764607523034234880u64, 
    11529215046068469760u64, 4611686018427387904u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0,
];

pub const KING_ATTACKS: [u64; 64] = [
    770u64, 1797u64, 3594u64, 7188u64, 14376u64, 28752u64, 57504u64, 49216u64, 197123u64, 460039u64, 920078u64, 1840156u64, 3680312u64, 7360624u64, 14721248u64, 12599488u64, 50463488u64, 117769984u64, 
    235539968u64, 471079936u64, 942159872u64, 1884319744u64, 3768639488u64, 3225468928u64, 12918652928u64, 30149115904u64, 60298231808u64, 120596463616u64, 241192927232u64, 482385854464u64, 
    964771708928u64, 825720045568u64, 3307175149568u64, 7718173671424u64, 15436347342848u64, 30872694685696u64, 61745389371392u64, 123490778742784u64, 246981557485568u64, 
    211384331665408u64, 846636838289408u64, 1975852459884544u64, 3951704919769088u64, 7903409839538176u64, 15806819679076352u64, 31613639358152704u64, 63227278716305408u64, 
    54114388906344448u64, 216739030602088448u64, 505818229730443264u64, 1011636459460886528u64, 2023272918921773056u64, 4046545837843546112u64, 8093091675687092224u64, 
    16186183351374184448u64, 13853283560024178688u64, 144959613005987840u64, 362258295026614272u64, 724516590053228544u64, 1449033180106457088u64, 2898066360212914176,
    5796132720425828352u64, 11592265440851656704u64, 4665729213955833856,
];

pub const RANK_MASK: [u64; 8] = [
    0b0000000000000000000000000000000000000000000000000000000011111111,
    0b0000000000000000000000000000000000000000000000001111111100000000,
    0b0000000000000000000000000000000000000000111111110000000000000000,
    0b0000000000000000000000000000000011111111000000000000000000000000,
    0b0000000000000000000000001111111100000000000000000000000000000000,
    0b0000000000000000111111110000000000000000000000000000000000000000,
    0b0000000011111111000000000000000000000000000000000000000000000000,
    0b1111111100000000000000000000000000000000000000000000000000000000,
];

const FILE_MASK: [u64; 8] = [
    0b0000000100000001000000010000000100000001000000010000000100000001,
    0b0000001000000010000000100000001000000010000000100000001000000010,
    0b0000010000000100000001000000010000000100000001000000010000000100,
    0b0000100000001000000010000000100000001000000010000000100000001000,
    0b0001000000010000000100000001000000010000000100000001000000010000,
    0b0010000000100000001000000010000000100000001000000010000000100000,
    0b0100000001000000010000000100000001000000010000000100000001000000,
    0b1000000010000000100000001000000010000000100000001000000010000000,
];
