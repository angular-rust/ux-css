//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/rgb/rgba/r.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  c-1: rgba(0,-1,0,1);\
            \n  c0: rgba(0,0,0,1);\
            \n  c1: rgba(0,1,0,1);\
            \n  c2: rgba(0,2,0,1);\
            \n  c3: rgba(0,3,0,1);\
            \n  c4: rgba(0,4,0,1);\
            \n  c5: rgba(0,5,0,1);\
            \n  c6: rgba(0,6,0,1);\
            \n  c7: rgba(0,7,0,1);\
            \n  c8: rgba(0,8,0,1);\
            \n  c9: rgba(0,9,0,1);\
            \n  c10: rgba(0,10,0,1);\
            \n  c11: rgba(0,11,0,1);\
            \n  c12: rgba(0,12,0,1);\
            \n  c13: rgba(0,13,0,1);\
            \n  c14: rgba(0,14,0,1);\
            \n  c15: rgba(0,15,0,1);\
            \n  c16: rgba(0,16,0,1);\
            \n  c17: rgba(0,17,0,1);\
            \n  c18: rgba(0,18,0,1);\
            \n  c19: rgba(0,19,0,1);\
            \n  c20: rgba(0,20,0,1);\
            \n  c21: rgba(0,21,0,1);\
            \n  c22: rgba(0,22,0,1);\
            \n  c23: rgba(0,23,0,1);\
            \n  c24: rgba(0,24,0,1);\
            \n  c25: rgba(0,25,0,1);\
            \n  c26: rgba(0,26,0,1);\
            \n  c27: rgba(0,27,0,1);\
            \n  c28: rgba(0,28,0,1);\
            \n  c29: rgba(0,29,0,1);\
            \n  c30: rgba(0,30,0,1);\
            \n  c31: rgba(0,31,0,1);\
            \n  c32: rgba(0,32,0,1);\
            \n  c33: rgba(0,33,0,1);\
            \n  c34: rgba(0,34,0,1);\
            \n  c35: rgba(0,35,0,1);\
            \n  c36: rgba(0,36,0,1);\
            \n  c37: rgba(0,37,0,1);\
            \n  c38: rgba(0,38,0,1);\
            \n  c39: rgba(0,39,0,1);\
            \n  c40: rgba(0,40,0,1);\
            \n  c41: rgba(0,41,0,1);\
            \n  c42: rgba(0,42,0,1);\
            \n  c43: rgba(0,43,0,1);\
            \n  c44: rgba(0,44,0,1);\
            \n  c45: rgba(0,45,0,1);\
            \n  c46: rgba(0,46,0,1);\
            \n  c47: rgba(0,47,0,1);\
            \n  c48: rgba(0,48,0,1);\
            \n  c49: rgba(0,49,0,1);\
            \n  c50: rgba(0,50,0,1);\
            \n  c51: rgba(0,51,0,1);\
            \n  c52: rgba(0,52,0,1);\
            \n  c53: rgba(0,53,0,1);\
            \n  c54: rgba(0,54,0,1);\
            \n  c55: rgba(0,55,0,1);\
            \n  c56: rgba(0,56,0,1);\
            \n  c57: rgba(0,57,0,1);\
            \n  c58: rgba(0,58,0,1);\
            \n  c59: rgba(0,59,0,1);\
            \n  c60: rgba(0,60,0,1);\
            \n  c61: rgba(0,61,0,1);\
            \n  c62: rgba(0,62,0,1);\
            \n  c63: rgba(0,63,0,1);\
            \n  c64: rgba(0,64,0,1);\
            \n  c65: rgba(0,65,0,1);\
            \n  c66: rgba(0,66,0,1);\
            \n  c67: rgba(0,67,0,1);\
            \n  c68: rgba(0,68,0,1);\
            \n  c69: rgba(0,69,0,1);\
            \n  c70: rgba(0,70,0,1);\
            \n  c71: rgba(0,71,0,1);\
            \n  c72: rgba(0,72,0,1);\
            \n  c73: rgba(0,73,0,1);\
            \n  c74: rgba(0,74,0,1);\
            \n  c75: rgba(0,75,0,1);\
            \n  c76: rgba(0,76,0,1);\
            \n  c77: rgba(0,77,0,1);\
            \n  c78: rgba(0,78,0,1);\
            \n  c79: rgba(0,79,0,1);\
            \n  c80: rgba(0,80,0,1);\
            \n  c81: rgba(0,81,0,1);\
            \n  c82: rgba(0,82,0,1);\
            \n  c83: rgba(0,83,0,1);\
            \n  c84: rgba(0,84,0,1);\
            \n  c85: rgba(0,85,0,1);\
            \n  c86: rgba(0,86,0,1);\
            \n  c87: rgba(0,87,0,1);\
            \n  c88: rgba(0,88,0,1);\
            \n  c89: rgba(0,89,0,1);\
            \n  c90: rgba(0,90,0,1);\
            \n  c91: rgba(0,91,0,1);\
            \n  c92: rgba(0,92,0,1);\
            \n  c93: rgba(0,93,0,1);\
            \n  c94: rgba(0,94,0,1);\
            \n  c95: rgba(0,95,0,1);\
            \n  c96: rgba(0,96,0,1);\
            \n  c97: rgba(0,97,0,1);\
            \n  c98: rgba(0,98,0,1);\
            \n  c99: rgba(0,99,0,1);\
            \n  c100: rgba(0,100,0,1);\
            \n  c101: rgba(0,101,0,1);\
            \n  c102: rgba(0,102,0,1);\
            \n  c103: rgba(0,103,0,1);\
            \n  c104: rgba(0,104,0,1);\
            \n  c105: rgba(0,105,0,1);\
            \n  c106: rgba(0,106,0,1);\
            \n  c107: rgba(0,107,0,1);\
            \n  c108: rgba(0,108,0,1);\
            \n  c109: rgba(0,109,0,1);\
            \n  c110: rgba(0,110,0,1);\
            \n  c111: rgba(0,111,0,1);\
            \n  c112: rgba(0,112,0,1);\
            \n  c113: rgba(0,113,0,1);\
            \n  c114: rgba(0,114,0,1);\
            \n  c115: rgba(0,115,0,1);\
            \n  c116: rgba(0,116,0,1);\
            \n  c117: rgba(0,117,0,1);\
            \n  c118: rgba(0,118,0,1);\
            \n  c119: rgba(0,119,0,1);\
            \n  c120: rgba(0,120,0,1);\
            \n  c121: rgba(0,121,0,1);\
            \n  c122: rgba(0,122,0,1);\
            \n  c123: rgba(0,123,0,1);\
            \n  c124: rgba(0,124,0,1);\
            \n  c125: rgba(0,125,0,1);\
            \n  c126: rgba(0,126,0,1);\
            \n  c127: rgba(0,127,0,1);\
            \n  c128: rgba(0,128,0,1);\
            \n  c129: rgba(0,129,0,1);\
            \n  c130: rgba(0,130,0,1);\
            \n  c131: rgba(0,131,0,1);\
            \n  c132: rgba(0,132,0,1);\
            \n  c133: rgba(0,133,0,1);\
            \n  c134: rgba(0,134,0,1);\
            \n  c135: rgba(0,135,0,1);\
            \n  c136: rgba(0,136,0,1);\
            \n  c137: rgba(0,137,0,1);\
            \n  c138: rgba(0,138,0,1);\
            \n  c139: rgba(0,139,0,1);\
            \n  c140: rgba(0,140,0,1);\
            \n  c141: rgba(0,141,0,1);\
            \n  c142: rgba(0,142,0,1);\
            \n  c143: rgba(0,143,0,1);\
            \n  c144: rgba(0,144,0,1);\
            \n  c145: rgba(0,145,0,1);\
            \n  c146: rgba(0,146,0,1);\
            \n  c147: rgba(0,147,0,1);\
            \n  c148: rgba(0,148,0,1);\
            \n  c149: rgba(0,149,0,1);\
            \n  c150: rgba(0,150,0,1);\
            \n  c151: rgba(0,151,0,1);\
            \n  c152: rgba(0,152,0,1);\
            \n  c153: rgba(0,153,0,1);\
            \n  c154: rgba(0,154,0,1);\
            \n  c155: rgba(0,155,0,1);\
            \n  c156: rgba(0,156,0,1);\
            \n  c157: rgba(0,157,0,1);\
            \n  c158: rgba(0,158,0,1);\
            \n  c159: rgba(0,159,0,1);\
            \n  c160: rgba(0,160,0,1);\
            \n  c161: rgba(0,161,0,1);\
            \n  c162: rgba(0,162,0,1);\
            \n  c163: rgba(0,163,0,1);\
            \n  c164: rgba(0,164,0,1);\
            \n  c165: rgba(0,165,0,1);\
            \n  c166: rgba(0,166,0,1);\
            \n  c167: rgba(0,167,0,1);\
            \n  c168: rgba(0,168,0,1);\
            \n  c169: rgba(0,169,0,1);\
            \n  c170: rgba(0,170,0,1);\
            \n  c171: rgba(0,171,0,1);\
            \n  c172: rgba(0,172,0,1);\
            \n  c173: rgba(0,173,0,1);\
            \n  c174: rgba(0,174,0,1);\
            \n  c175: rgba(0,175,0,1);\
            \n  c176: rgba(0,176,0,1);\
            \n  c177: rgba(0,177,0,1);\
            \n  c178: rgba(0,178,0,1);\
            \n  c179: rgba(0,179,0,1);\
            \n  c180: rgba(0,180,0,1);\
            \n  c181: rgba(0,181,0,1);\
            \n  c182: rgba(0,182,0,1);\
            \n  c183: rgba(0,183,0,1);\
            \n  c184: rgba(0,184,0,1);\
            \n  c185: rgba(0,185,0,1);\
            \n  c186: rgba(0,186,0,1);\
            \n  c187: rgba(0,187,0,1);\
            \n  c188: rgba(0,188,0,1);\
            \n  c189: rgba(0,189,0,1);\
            \n  c190: rgba(0,190,0,1);\
            \n  c191: rgba(0,191,0,1);\
            \n  c192: rgba(0,192,0,1);\
            \n  c193: rgba(0,193,0,1);\
            \n  c194: rgba(0,194,0,1);\
            \n  c195: rgba(0,195,0,1);\
            \n  c196: rgba(0,196,0,1);\
            \n  c197: rgba(0,197,0,1);\
            \n  c198: rgba(0,198,0,1);\
            \n  c199: rgba(0,199,0,1);\
            \n  c200: rgba(0,200,0,1);\
            \n  c201: rgba(0,201,0,1);\
            \n  c202: rgba(0,202,0,1);\
            \n  c203: rgba(0,203,0,1);\
            \n  c204: rgba(0,204,0,1);\
            \n  c205: rgba(0,205,0,1);\
            \n  c206: rgba(0,206,0,1);\
            \n  c207: rgba(0,207,0,1);\
            \n  c208: rgba(0,208,0,1);\
            \n  c209: rgba(0,209,0,1);\
            \n  c210: rgba(0,210,0,1);\
            \n  c211: rgba(0,211,0,1);\
            \n  c212: rgba(0,212,0,1);\
            \n  c213: rgba(0,213,0,1);\
            \n  c214: rgba(0,214,0,1);\
            \n  c215: rgba(0,215,0,1);\
            \n  c216: rgba(0,216,0,1);\
            \n  c217: rgba(0,217,0,1);\
            \n  c218: rgba(0,218,0,1);\
            \n  c219: rgba(0,219,0,1);\
            \n  c220: rgba(0,220,0,1);\
            \n  c221: rgba(0,221,0,1);\
            \n  c222: rgba(0,222,0,1);\
            \n  c223: rgba(0,223,0,1);\
            \n  c224: rgba(0,224,0,1);\
            \n  c225: rgba(0,225,0,1);\
            \n  c226: rgba(0,226,0,1);\
            \n  c227: rgba(0,227,0,1);\
            \n  c228: rgba(0,228,0,1);\
            \n  c229: rgba(0,229,0,1);\
            \n  c230: rgba(0,230,0,1);\
            \n  c231: rgba(0,231,0,1);\
            \n  c232: rgba(0,232,0,1);\
            \n  c233: rgba(0,233,0,1);\
            \n  c234: rgba(0,234,0,1);\
            \n  c235: rgba(0,235,0,1);\
            \n  c236: rgba(0,236,0,1);\
            \n  c237: rgba(0,237,0,1);\
            \n  c238: rgba(0,238,0,1);\
            \n  c239: rgba(0,239,0,1);\
            \n  c240: rgba(0,240,0,1);\
            \n  c241: rgba(0,241,0,1);\
            \n  c242: rgba(0,242,0,1);\
            \n  c243: rgba(0,243,0,1);\
            \n  c244: rgba(0,244,0,1);\
            \n  c245: rgba(0,245,0,1);\
            \n  c246: rgba(0,246,0,1);\
            \n  c247: rgba(0,247,0,1);\
            \n  c248: rgba(0,248,0,1);\
            \n  c249: rgba(0,249,0,1);\
            \n  c250: rgba(0,250,0,1);\
            \n  c251: rgba(0,251,0,1);\
            \n  c252: rgba(0,252,0,1);\
            \n  c253: rgba(0,253,0,1);\
            \n  c254: rgba(0,254,0,1);\
            \n  c255: rgba(0,255,0,1);\
            \n  c256: rgba(0,256,0,1);\
            \n}\
            \n\
            \nfoo {\
            \n  c-1: rgba(-1%,0,0,1);\
            \n  c0: rgba(0%,0,0,1);\
            \n  c1: rgba(1%,0,0,1);\
            \n  c2: rgba(2%,0,0,1);\
            \n  c3: rgba(3%,0,0,1);\
            \n  c4: rgba(4%,0,0,1);\
            \n  c5: rgba(5%,0,0,1);\
            \n  c6: rgba(6%,0,0,1);\
            \n  c7: rgba(7%,0,0,1);\
            \n  c8: rgba(8%,0,0,1);\
            \n  c9: rgba(9%,0,0,1);\
            \n  c10: rgba(10%,0,0,1);\
            \n  c11: rgba(11%,0,0,1);\
            \n  c12: rgba(12%,0,0,1);\
            \n  c13: rgba(13%,0,0,1);\
            \n  c14: rgba(14%,0,0,1);\
            \n  c15: rgba(15%,0,0,1);\
            \n  c16: rgba(16%,0,0,1);\
            \n  c17: rgba(17%,0,0,1);\
            \n  c18: rgba(18%,0,0,1);\
            \n  c19: rgba(19%,0,0,1);\
            \n  c20: rgba(20%,0,0,1);\
            \n  c21: rgba(21%,0,0,1);\
            \n  c22: rgba(22%,0,0,1);\
            \n  c23: rgba(23%,0,0,1);\
            \n  c24: rgba(24%,0,0,1);\
            \n  c25: rgba(25%,0,0,1);\
            \n  c26: rgba(26%,0,0,1);\
            \n  c27: rgba(27%,0,0,1);\
            \n  c28: rgba(28%,0,0,1);\
            \n  c29: rgba(29%,0,0,1);\
            \n  c30: rgba(30%,0,0,1);\
            \n  c31: rgba(31%,0,0,1);\
            \n  c32: rgba(32%,0,0,1);\
            \n  c33: rgba(33%,0,0,1);\
            \n  c34: rgba(34%,0,0,1);\
            \n  c35: rgba(35%,0,0,1);\
            \n  c36: rgba(36%,0,0,1);\
            \n  c37: rgba(37%,0,0,1);\
            \n  c38: rgba(38%,0,0,1);\
            \n  c39: rgba(39%,0,0,1);\
            \n  c40: rgba(40%,0,0,1);\
            \n  c41: rgba(41%,0,0,1);\
            \n  c42: rgba(42%,0,0,1);\
            \n  c43: rgba(43%,0,0,1);\
            \n  c44: rgba(44%,0,0,1);\
            \n  c45: rgba(45%,0,0,1);\
            \n  c46: rgba(46%,0,0,1);\
            \n  c47: rgba(47%,0,0,1);\
            \n  c48: rgba(48%,0,0,1);\
            \n  c49: rgba(49%,0,0,1);\
            \n  c50: rgba(50%,0,0,1);\
            \n  c51: rgba(51%,0,0,1);\
            \n  c52: rgba(52%,0,0,1);\
            \n  c53: rgba(53%,0,0,1);\
            \n  c54: rgba(54%,0,0,1);\
            \n  c55: rgba(55%,0,0,1);\
            \n  c56: rgba(56%,0,0,1);\
            \n  c57: rgba(57%,0,0,1);\
            \n  c58: rgba(58%,0,0,1);\
            \n  c59: rgba(59%,0,0,1);\
            \n  c60: rgba(60%,0,0,1);\
            \n  c61: rgba(61%,0,0,1);\
            \n  c62: rgba(62%,0,0,1);\
            \n  c63: rgba(63%,0,0,1);\
            \n  c64: rgba(64%,0,0,1);\
            \n  c65: rgba(65%,0,0,1);\
            \n  c66: rgba(66%,0,0,1);\
            \n  c67: rgba(67%,0,0,1);\
            \n  c68: rgba(68%,0,0,1);\
            \n  c69: rgba(69%,0,0,1);\
            \n  c70: rgba(70%,0,0,1);\
            \n  c71: rgba(71%,0,0,1);\
            \n  c72: rgba(72%,0,0,1);\
            \n  c73: rgba(73%,0,0,1);\
            \n  c74: rgba(74%,0,0,1);\
            \n  c75: rgba(75%,0,0,1);\
            \n  c76: rgba(76%,0,0,1);\
            \n  c77: rgba(77%,0,0,1);\
            \n  c78: rgba(78%,0,0,1);\
            \n  c79: rgba(79%,0,0,1);\
            \n  c80: rgba(80%,0,0,1);\
            \n  c81: rgba(81%,0,0,1);\
            \n  c82: rgba(82%,0,0,1);\
            \n  c83: rgba(83%,0,0,1);\
            \n  c84: rgba(84%,0,0,1);\
            \n  c85: rgba(85%,0,0,1);\
            \n  c86: rgba(86%,0,0,1);\
            \n  c87: rgba(87%,0,0,1);\
            \n  c88: rgba(88%,0,0,1);\
            \n  c89: rgba(89%,0,0,1);\
            \n  c90: rgba(90%,0,0,1);\
            \n  c91: rgba(91%,0,0,1);\
            \n  c92: rgba(92%,0,0,1);\
            \n  c93: rgba(93%,0,0,1);\
            \n  c94: rgba(94%,0,0,1);\
            \n  c95: rgba(95%,0,0,1);\
            \n  c96: rgba(96%,0,0,1);\
            \n  c97: rgba(97%,0,0,1);\
            \n  c98: rgba(98%,0,0,1);\
            \n  c99: rgba(99%,0,0,1);\
            \n  c100: rgba(100%,0,0,1);\
            \n  c101: rgba(101%,0,0,1);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  c-1: black;\
        \n  c0: black;\
        \n  c1: #000100;\
        \n  c2: #000200;\
        \n  c3: #000300;\
        \n  c4: #000400;\
        \n  c5: #000500;\
        \n  c6: #000600;\
        \n  c7: #000700;\
        \n  c8: #000800;\
        \n  c9: #000900;\
        \n  c10: #000a00;\
        \n  c11: #000b00;\
        \n  c12: #000c00;\
        \n  c13: #000d00;\
        \n  c14: #000e00;\
        \n  c15: #000f00;\
        \n  c16: #001000;\
        \n  c17: #001100;\
        \n  c18: #001200;\
        \n  c19: #001300;\
        \n  c20: #001400;\
        \n  c21: #001500;\
        \n  c22: #001600;\
        \n  c23: #001700;\
        \n  c24: #001800;\
        \n  c25: #001900;\
        \n  c26: #001a00;\
        \n  c27: #001b00;\
        \n  c28: #001c00;\
        \n  c29: #001d00;\
        \n  c30: #001e00;\
        \n  c31: #001f00;\
        \n  c32: #002000;\
        \n  c33: #002100;\
        \n  c34: #002200;\
        \n  c35: #002300;\
        \n  c36: #002400;\
        \n  c37: #002500;\
        \n  c38: #002600;\
        \n  c39: #002700;\
        \n  c40: #002800;\
        \n  c41: #002900;\
        \n  c42: #002a00;\
        \n  c43: #002b00;\
        \n  c44: #002c00;\
        \n  c45: #002d00;\
        \n  c46: #002e00;\
        \n  c47: #002f00;\
        \n  c48: #003000;\
        \n  c49: #003100;\
        \n  c50: #003200;\
        \n  c51: #003300;\
        \n  c52: #003400;\
        \n  c53: #003500;\
        \n  c54: #003600;\
        \n  c55: #003700;\
        \n  c56: #003800;\
        \n  c57: #003900;\
        \n  c58: #003a00;\
        \n  c59: #003b00;\
        \n  c60: #003c00;\
        \n  c61: #003d00;\
        \n  c62: #003e00;\
        \n  c63: #003f00;\
        \n  c64: #004000;\
        \n  c65: #004100;\
        \n  c66: #004200;\
        \n  c67: #004300;\
        \n  c68: #004400;\
        \n  c69: #004500;\
        \n  c70: #004600;\
        \n  c71: #004700;\
        \n  c72: #004800;\
        \n  c73: #004900;\
        \n  c74: #004a00;\
        \n  c75: #004b00;\
        \n  c76: #004c00;\
        \n  c77: #004d00;\
        \n  c78: #004e00;\
        \n  c79: #004f00;\
        \n  c80: #005000;\
        \n  c81: #005100;\
        \n  c82: #005200;\
        \n  c83: #005300;\
        \n  c84: #005400;\
        \n  c85: #005500;\
        \n  c86: #005600;\
        \n  c87: #005700;\
        \n  c88: #005800;\
        \n  c89: #005900;\
        \n  c90: #005a00;\
        \n  c91: #005b00;\
        \n  c92: #005c00;\
        \n  c93: #005d00;\
        \n  c94: #005e00;\
        \n  c95: #005f00;\
        \n  c96: #006000;\
        \n  c97: #006100;\
        \n  c98: #006200;\
        \n  c99: #006300;\
        \n  c100: darkgreen;\
        \n  c101: #006500;\
        \n  c102: #006600;\
        \n  c103: #006700;\
        \n  c104: #006800;\
        \n  c105: #006900;\
        \n  c106: #006a00;\
        \n  c107: #006b00;\
        \n  c108: #006c00;\
        \n  c109: #006d00;\
        \n  c110: #006e00;\
        \n  c111: #006f00;\
        \n  c112: #007000;\
        \n  c113: #007100;\
        \n  c114: #007200;\
        \n  c115: #007300;\
        \n  c116: #007400;\
        \n  c117: #007500;\
        \n  c118: #007600;\
        \n  c119: #007700;\
        \n  c120: #007800;\
        \n  c121: #007900;\
        \n  c122: #007a00;\
        \n  c123: #007b00;\
        \n  c124: #007c00;\
        \n  c125: #007d00;\
        \n  c126: #007e00;\
        \n  c127: #007f00;\
        \n  c128: green;\
        \n  c129: #008100;\
        \n  c130: #008200;\
        \n  c131: #008300;\
        \n  c132: #008400;\
        \n  c133: #008500;\
        \n  c134: #008600;\
        \n  c135: #008700;\
        \n  c136: #008800;\
        \n  c137: #008900;\
        \n  c138: #008a00;\
        \n  c139: #008b00;\
        \n  c140: #008c00;\
        \n  c141: #008d00;\
        \n  c142: #008e00;\
        \n  c143: #008f00;\
        \n  c144: #009000;\
        \n  c145: #009100;\
        \n  c146: #009200;\
        \n  c147: #009300;\
        \n  c148: #009400;\
        \n  c149: #009500;\
        \n  c150: #009600;\
        \n  c151: #009700;\
        \n  c152: #009800;\
        \n  c153: #009900;\
        \n  c154: #009a00;\
        \n  c155: #009b00;\
        \n  c156: #009c00;\
        \n  c157: #009d00;\
        \n  c158: #009e00;\
        \n  c159: #009f00;\
        \n  c160: #00a000;\
        \n  c161: #00a100;\
        \n  c162: #00a200;\
        \n  c163: #00a300;\
        \n  c164: #00a400;\
        \n  c165: #00a500;\
        \n  c166: #00a600;\
        \n  c167: #00a700;\
        \n  c168: #00a800;\
        \n  c169: #00a900;\
        \n  c170: #00aa00;\
        \n  c171: #00ab00;\
        \n  c172: #00ac00;\
        \n  c173: #00ad00;\
        \n  c174: #00ae00;\
        \n  c175: #00af00;\
        \n  c176: #00b000;\
        \n  c177: #00b100;\
        \n  c178: #00b200;\
        \n  c179: #00b300;\
        \n  c180: #00b400;\
        \n  c181: #00b500;\
        \n  c182: #00b600;\
        \n  c183: #00b700;\
        \n  c184: #00b800;\
        \n  c185: #00b900;\
        \n  c186: #00ba00;\
        \n  c187: #00bb00;\
        \n  c188: #00bc00;\
        \n  c189: #00bd00;\
        \n  c190: #00be00;\
        \n  c191: #00bf00;\
        \n  c192: #00c000;\
        \n  c193: #00c100;\
        \n  c194: #00c200;\
        \n  c195: #00c300;\
        \n  c196: #00c400;\
        \n  c197: #00c500;\
        \n  c198: #00c600;\
        \n  c199: #00c700;\
        \n  c200: #00c800;\
        \n  c201: #00c900;\
        \n  c202: #00ca00;\
        \n  c203: #00cb00;\
        \n  c204: #00cc00;\
        \n  c205: #00cd00;\
        \n  c206: #00ce00;\
        \n  c207: #00cf00;\
        \n  c208: #00d000;\
        \n  c209: #00d100;\
        \n  c210: #00d200;\
        \n  c211: #00d300;\
        \n  c212: #00d400;\
        \n  c213: #00d500;\
        \n  c214: #00d600;\
        \n  c215: #00d700;\
        \n  c216: #00d800;\
        \n  c217: #00d900;\
        \n  c218: #00da00;\
        \n  c219: #00db00;\
        \n  c220: #00dc00;\
        \n  c221: #00dd00;\
        \n  c222: #00de00;\
        \n  c223: #00df00;\
        \n  c224: #00e000;\
        \n  c225: #00e100;\
        \n  c226: #00e200;\
        \n  c227: #00e300;\
        \n  c228: #00e400;\
        \n  c229: #00e500;\
        \n  c230: #00e600;\
        \n  c231: #00e700;\
        \n  c232: #00e800;\
        \n  c233: #00e900;\
        \n  c234: #00ea00;\
        \n  c235: #00eb00;\
        \n  c236: #00ec00;\
        \n  c237: #00ed00;\
        \n  c238: #00ee00;\
        \n  c239: #00ef00;\
        \n  c240: #00f000;\
        \n  c241: #00f100;\
        \n  c242: #00f200;\
        \n  c243: #00f300;\
        \n  c244: #00f400;\
        \n  c245: #00f500;\
        \n  c246: #00f600;\
        \n  c247: #00f700;\
        \n  c248: #00f800;\
        \n  c249: #00f900;\
        \n  c250: #00fa00;\
        \n  c251: #00fb00;\
        \n  c252: #00fc00;\
        \n  c253: #00fd00;\
        \n  c254: #00fe00;\
        \n  c255: lime;\
        \n  c256: lime;\
        \n}\
        \nfoo {\
        \n  c-1: black;\
        \n  c0: black;\
        \n  c1: #030000;\
        \n  c2: #050000;\
        \n  c3: #080000;\
        \n  c4: #0a0000;\
        \n  c5: #0d0000;\
        \n  c6: #0f0000;\
        \n  c7: #120000;\
        \n  c8: #140000;\
        \n  c9: #170000;\
        \n  c10: #1a0000;\
        \n  c11: #1c0000;\
        \n  c12: #1f0000;\
        \n  c13: #210000;\
        \n  c14: #240000;\
        \n  c15: #260000;\
        \n  c16: #290000;\
        \n  c17: #2b0000;\
        \n  c18: #2e0000;\
        \n  c19: #300000;\
        \n  c20: #330000;\
        \n  c21: #360000;\
        \n  c22: #380000;\
        \n  c23: #3b0000;\
        \n  c24: #3d0000;\
        \n  c25: #400000;\
        \n  c26: #420000;\
        \n  c27: #450000;\
        \n  c28: #470000;\
        \n  c29: #4a0000;\
        \n  c30: #4d0000;\
        \n  c31: #4f0000;\
        \n  c32: #520000;\
        \n  c33: #540000;\
        \n  c34: #570000;\
        \n  c35: #590000;\
        \n  c36: #5c0000;\
        \n  c37: #5e0000;\
        \n  c38: #610000;\
        \n  c39: #630000;\
        \n  c40: #660000;\
        \n  c41: #690000;\
        \n  c42: #6b0000;\
        \n  c43: #6e0000;\
        \n  c44: #700000;\
        \n  c45: #730000;\
        \n  c46: #750000;\
        \n  c47: #780000;\
        \n  c48: #7a0000;\
        \n  c49: #7d0000;\
        \n  c50: maroon;\
        \n  c51: #820000;\
        \n  c52: #850000;\
        \n  c53: #870000;\
        \n  c54: #8a0000;\
        \n  c55: #8c0000;\
        \n  c56: #8f0000;\
        \n  c57: #910000;\
        \n  c58: #940000;\
        \n  c59: #960000;\
        \n  c60: #990000;\
        \n  c61: #9c0000;\
        \n  c62: #9e0000;\
        \n  c63: #a10000;\
        \n  c64: #a30000;\
        \n  c65: #a60000;\
        \n  c66: #a80000;\
        \n  c67: #ab0000;\
        \n  c68: #ad0000;\
        \n  c69: #b00000;\
        \n  c70: #b30000;\
        \n  c71: #b50000;\
        \n  c72: #b80000;\
        \n  c73: #ba0000;\
        \n  c74: #bd0000;\
        \n  c75: #bf0000;\
        \n  c76: #c20000;\
        \n  c77: #c40000;\
        \n  c78: #c70000;\
        \n  c79: #c90000;\
        \n  c80: #cc0000;\
        \n  c81: #cf0000;\
        \n  c82: #d10000;\
        \n  c83: #d40000;\
        \n  c84: #d60000;\
        \n  c85: #d90000;\
        \n  c86: #db0000;\
        \n  c87: #de0000;\
        \n  c88: #e00000;\
        \n  c89: #e30000;\
        \n  c90: #e60000;\
        \n  c91: #e80000;\
        \n  c92: #eb0000;\
        \n  c93: #ed0000;\
        \n  c94: #f00000;\
        \n  c95: #f20000;\
        \n  c96: #f50000;\
        \n  c97: #f70000;\
        \n  c98: #fa0000;\
        \n  c99: #fc0000;\
        \n  c100: red;\
        \n  c101: red;\
        \n}\
        \n"
    );
}
