pub const EOL: &str = "\n";
pub const GS: u8 = 0x1D;
pub const ESC: u8 = 0x1B;
pub const NIL: u8 = 0x00;

pub const HARDWARE_INIT: &[u8] = &[ESC, 0x40];
pub const HARDWARE_SELECT: &[u8] = &[ESC, 0x3D, 0x01];
pub const HARDWARE_RESET: &[u8] = &[ESC, 0x3F, 0x0A, 0x00];
pub const HARDWARE_PRINT_MODE: &[u8] = &[ESC, 0x21];
// Paper feed

pub const PAPER_FEED_FORWARD: &[u8] = &[ESC, 0x64];
pub const PAPER_FEED_REVERSE: &[u8] = &[ESC, 0x65];

pub const PAGE_MODE_STANDARD: &[u8] = &[ESC, 0x4C];

// Paper cut
pub const PAPER_CUT_FULL: &[u8] = &[GS, 0x56, 0x00];
pub const PAPER_CUT_PARTIAL: &[u8] = &[GS, 0x56, 0x01];
pub const PAPER_CUT_PARTIAL_A: &[u8] = &[GS, 0x56, 0x41];
pub const PAPER_CUT_PARTIAL_B: &[u8] = &[GS, 0x56, 0x42];

// Text styles
pub const TEXT_BOLD_MODE_OFF: &[u8] = &[ESC, 0x45, 0x00];
pub const TEXT_BOLD_MODE_ON: &[u8] = &[ESC, 0x45, 0x01];
pub const TEXT_UNDERLINE_MODE_OFF: &[u8] = &[ESC, 0x2D, 0x00];
pub const TEXT_UNDERLINE_MODE_ON: &[u8] = &[ESC, 0x2D, 0x01];
pub const TEXT_UNDERLINE_MODE_2_ON: &[u8] = &[ESC, 0x2D, 0x02];
pub const TEXT_DOUBLESTRIKE_OFF: &[u8] = &[ESC, 0x47, 0x00];
pub const TEXT_DOUBLESTRIKE_ON: &[u8] = &[ESC, 0x47, 0x01];
pub const TEXT_FLIP_OFF: &[u8] = &[ESC, 0x7B, 0x00];
pub const TEXT_FLIP_ON: &[u8] = &[ESC, 0x7B, 0x01];
pub const TEXT_REVERSE_COLOURS_ON: &[u8] = &[GS, 0x42, 0x01];
pub const TEXT_REVERSE_COLOURS_OFF: &[u8] = &[GS, 0x42, 0x00];

// Text alignment
pub const TEXT_JUSTIFY_LEFT: &[u8] = &[ESC, 0x61, 0x00];
pub const TEXT_JUSTIFY_CENTER: &[u8] = &[ESC, 0x61, 0x01];
pub const TEXT_JUSTIFY_RIGHT: &[u8] = &[ESC, 0x61, 0x02];
pub const TEXT_MARGIN_LEFT: &[u8] = &[GS, 0x4C];
pub const TEXT_PRINTABLE_AREA: &[u8] = &[GS, 0x57];

// Font selection
pub const FONT_A: &[u8] = &[ESC, 0x4D, 0x00];
pub const FONT_B: &[u8] = &[ESC, 0x4D, 0x01];
pub const FONT_C: &[u8] = &[ESC, 0x4D, 0x02];

// Barcode commands
pub const BARCODE_HEIGHT: &[u8] = &[GS, 0x68];
pub const BARCODE_WIDTH: &[u8] = &[GS, 0x77];
pub const BARCODE_TEXT_POSITION: &[u8] = &[GS, 0x48];

pub const BARCODE_FONT_A: &[u8] = &[GS, 0x66, 0x00];
pub const BARCODE_FONT_B: &[u8] = &[GS, 0x66, 0x01];

pub const BARCODE_TEXT_NONE: &[u8] = &[GS, 0x48, 0x00];
pub const BARCODE_TEXT_ABOVE: &[u8] = &[GS, 0x48, 0x01];
pub const BARCODE_TEXT_BELOW: &[u8] = &[GS, 0x48, 0x02];
pub const BARCODE_TEXT_BOTH: &[u8] = &[GS, 0x48, 0x03];

pub const BARCODE_TYPE_UPC_A: &[u8] = &[GS, 0x6B, 0x00];
pub const BARCODE_TYPE_UPC_E: &[u8] = &[GS, 0x6B, 0x01];
pub const BARCODE_TYPE_JAN13_EAN13: &[u8] = &[GS, 0x6B, 0x02];
pub const BARCODE_TYPE_JAN8_EAN8: &[u8] = &[GS, 0x6B, 0x03];
pub const BARCODE_TYPE_CODE39: &[u8] = &[GS, 0x6B, 0x04];
pub const BARCODE_TYPE_ITF: &[u8] = &[GS, 0x6B, 0x05];
pub const BARCODE_TYPE_CODABAR: &[u8] = &[GS, 0x6B, 0x06];

// QR Code commands

pub const QR_CORRECTION_ERROR_LEVEL_LOW: u8 = 0x30;
pub const QR_CORRECTION_ERROR_LEVEL_MEDIUM: u8 = 0x31;
pub const QR_CORRECTION_ERROR_LEVEL_QUARTILE: u8 = 0x32;
pub const QR_CORRECTION_ERROR_LEVEL_HIGH: u8 = 0x33;

pub const QR_MODEL_1: u8 = 0x31;
pub const QR_MODEL_2: u8 = 0x32;

pub const QR_SELECT_MODEL: &[u8] = &[GS, 0x28, 0x6B, 0x04, 0x00, 0x31, 0x41, 0x00];
pub const QR_LEVEL: &[u8] = &[GS, 0x28, 0x6B, 0x03, 0x00, 0x31, 0x43];
pub const QR_CORRECTION_ERROR_LEVEL: &[u8] = &[GS, 0x28, 0x6B, 0x03, 0x00, 0x31, 0x45];
pub const QR_DATA_STORE_PREFIX: &[u8] = &[GS, 0x28, 0x6B];
pub const QR_DATA_STORE_SUFFIX: &[u8] = &[0x31, 0x50, 0x30];
pub const QR_PRINT: &[u8] = &[GS, 0x28, 0x6B, 0x03, 0x00, 0x31, 0x51, 0x30];

// Image

pub const IMAGE_HEADER: &[u8] = &[GS, 0x76, 0x30];
