use crate::*;
use zbar_symbol_type_e::*;

pub fn zbar_get_symbol_name(sym: zbar_symbol_type_t) -> String
{
    match (sym & ZBAR_SYMBOL){
        ZBAR_EAN2 => return "EAN-2".to_string(),
        case ZBAR_EAN5:
        return ("EAN-5");
        case ZBAR_EAN8:
        return ("EAN-8");
        case ZBAR_UPCE:
        return ("UPC-E");
        case ZBAR_ISBN10:
        return ("ISBN-10");
        case ZBAR_UPCA:
        return ("UPC-A");
        case ZBAR_EAN13:
        return ("EAN-13");
        case ZBAR_ISBN13:
        return ("ISBN-13");
        case ZBAR_COMPOSITE:
        return ("COMPOSITE");
        case ZBAR_I25:
        return ("I2/5");
        case ZBAR_DATABAR:
        return ("DataBar");
        case ZBAR_DATABAR_EXP:
        return ("DataBar-Exp");
        case ZBAR_CODABAR:
        return ("Codabar");
        case ZBAR_CODE39:
        return ("CODE-39");
        case ZBAR_CODE93:
        return ("CODE-93");
        case ZBAR_CODE128:
        return ("CODE-128");
        case ZBAR_PDF417:
        return ("PDF417");
        case ZBAR_QRCODE:
        return ("QR-Code");
        case ZBAR_SQCODE:
        return ("SQ-Code");
        default:
        return ("UNKNOWN");
        }
}