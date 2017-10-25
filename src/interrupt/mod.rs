use bare_metal::Nr;
#[cfg(all(target_arch = "arm", feature = "rt"))]
global_asm!("\n                    .thumb_func\n                    DH_TRAMPOLINE:\n                        b DEFAULT_HANDLER\n                    ");
#[doc = r" Hack to compile on x86"]
#[cfg(all(target_arch = "x86_64", feature = "rt"))]
global_asm!("\n                    DH_TRAMPOLINE:\n                        jmp DEFAULT_HANDLER\n                    ");
#[cfg(feature = "rt")]
global_asm!(
    "\n.weak SCU_0\nSCU_0 = DH_TRAMPOLINE\n.weak ERU0_0\nERU0_0 = DH_TRAMPOLINE\n.weak ERU0_1\nERU0_1 = DH_TRAMPOLINE\n.weak ERU0_2\nERU0_2 = DH_TRAMPOLINE\n.weak ERU0_3\nERU0_3 = DH_TRAMPOLINE\n.weak ERU1_0\nERU1_0 = DH_TRAMPOLINE\n.weak ERU1_1\nERU1_1 = DH_TRAMPOLINE\n.weak ERU1_2\nERU1_2 = DH_TRAMPOLINE\n.weak ERU1_3\nERU1_3 = DH_TRAMPOLINE\n.weak PMU0_0\nPMU0_0 = DH_TRAMPOLINE\n.weak VADC0_C0_0\nVADC0_C0_0 = DH_TRAMPOLINE\n.weak VADC0_C0_1\nVADC0_C0_1 = DH_TRAMPOLINE\n.weak VADC0_C0_2\nVADC0_C0_2 = DH_TRAMPOLINE\n.weak VADC0_C0_3\nVADC0_C0_3 = DH_TRAMPOLINE\n.weak VADC0_G0_0\nVADC0_G0_0 = DH_TRAMPOLINE\n.weak VADC0_G0_1\nVADC0_G0_1 = DH_TRAMPOLINE\n.weak VADC0_G0_2\nVADC0_G0_2 = DH_TRAMPOLINE\n.weak VADC0_G0_3\nVADC0_G0_3 = DH_TRAMPOLINE\n.weak VADC0_G1_0\nVADC0_G1_0 = DH_TRAMPOLINE\n.weak VADC0_G1_1\nVADC0_G1_1 = DH_TRAMPOLINE\n.weak VADC0_G1_2\nVADC0_G1_2 = DH_TRAMPOLINE\n.weak VADC0_G1_3\nVADC0_G1_3 = DH_TRAMPOLINE\n.weak DAC0_0\nDAC0_0 = DH_TRAMPOLINE\n.weak DAC0_1\nDAC0_1 = DH_TRAMPOLINE\n.weak CCU40_0\nCCU40_0 = DH_TRAMPOLINE\n.weak CCU40_1\nCCU40_1 = DH_TRAMPOLINE\n.weak CCU40_2\nCCU40_2 = DH_TRAMPOLINE\n.weak CCU40_3\nCCU40_3 = DH_TRAMPOLINE\n.weak CCU41_0\nCCU41_0 = DH_TRAMPOLINE\n.weak CCU41_1\nCCU41_1 = DH_TRAMPOLINE\n.weak CCU41_2\nCCU41_2 = DH_TRAMPOLINE\n.weak CCU41_3\nCCU41_3 = DH_TRAMPOLINE\n.weak CCU80_0\nCCU80_0 = DH_TRAMPOLINE\n.weak CCU80_1\nCCU80_1 = DH_TRAMPOLINE\n.weak CCU80_2\nCCU80_2 = DH_TRAMPOLINE\n.weak CCU80_3\nCCU80_3 = DH_TRAMPOLINE\n.weak POSIF0_0\nPOSIF0_0 = DH_TRAMPOLINE\n.weak POSIF0_1\nPOSIF0_1 = DH_TRAMPOLINE\n.weak HRPWM_0\nHRPWM_0 = DH_TRAMPOLINE\n.weak HRPWM_1\nHRPWM_1 = DH_TRAMPOLINE\n.weak HRPWM_2\nHRPWM_2 = DH_TRAMPOLINE\n.weak HRPWM_3\nHRPWM_3 = DH_TRAMPOLINE\n.weak CAN0_0\nCAN0_0 = DH_TRAMPOLINE\n.weak CAN0_1\nCAN0_1 = DH_TRAMPOLINE\n.weak CAN0_2\nCAN0_2 = DH_TRAMPOLINE\n.weak CAN0_3\nCAN0_3 = DH_TRAMPOLINE\n.weak CAN0_4\nCAN0_4 = DH_TRAMPOLINE\n.weak CAN0_5\nCAN0_5 = DH_TRAMPOLINE\n.weak CAN0_6\nCAN0_6 = DH_TRAMPOLINE\n.weak CAN0_7\nCAN0_7 = DH_TRAMPOLINE\n.weak USIC0_0\nUSIC0_0 = DH_TRAMPOLINE\n.weak USIC0_1\nUSIC0_1 = DH_TRAMPOLINE\n.weak USIC0_2\nUSIC0_2 = DH_TRAMPOLINE\n.weak USIC0_3\nUSIC0_3 = DH_TRAMPOLINE\n.weak USIC0_4\nUSIC0_4 = DH_TRAMPOLINE\n.weak USIC0_5\nUSIC0_5 = DH_TRAMPOLINE\n.weak USIC1_0\nUSIC1_0 = DH_TRAMPOLINE\n.weak USIC1_1\nUSIC1_1 = DH_TRAMPOLINE\n.weak USIC1_2\nUSIC1_2 = DH_TRAMPOLINE\n.weak USIC1_3\nUSIC1_3 = DH_TRAMPOLINE\n.weak USIC1_4\nUSIC1_4 = DH_TRAMPOLINE\n.weak USIC1_5\nUSIC1_5 = DH_TRAMPOLINE\n.weak LEDTS0_0\nLEDTS0_0 = DH_TRAMPOLINE\n.weak FCE0_0\nFCE0_0 = DH_TRAMPOLINE\n.weak GPDMA0_0\nGPDMA0_0 = DH_TRAMPOLINE\n.weak USB0_0\nUSB0_0 = DH_TRAMPOLINE"
);
#[cfg(feature = "rt")]
extern "C" {
    fn SCU_0();
    fn ERU0_0();
    fn ERU0_1();
    fn ERU0_2();
    fn ERU0_3();
    fn ERU1_0();
    fn ERU1_1();
    fn ERU1_2();
    fn ERU1_3();
    fn PMU0_0();
    fn VADC0_C0_0();
    fn VADC0_C0_1();
    fn VADC0_C0_2();
    fn VADC0_C0_3();
    fn VADC0_G0_0();
    fn VADC0_G0_1();
    fn VADC0_G0_2();
    fn VADC0_G0_3();
    fn VADC0_G1_0();
    fn VADC0_G1_1();
    fn VADC0_G1_2();
    fn VADC0_G1_3();
    fn DAC0_0();
    fn DAC0_1();
    fn CCU40_0();
    fn CCU40_1();
    fn CCU40_2();
    fn CCU40_3();
    fn CCU41_0();
    fn CCU41_1();
    fn CCU41_2();
    fn CCU41_3();
    fn CCU80_0();
    fn CCU80_1();
    fn CCU80_2();
    fn CCU80_3();
    fn POSIF0_0();
    fn POSIF0_1();
    fn HRPWM_0();
    fn HRPWM_1();
    fn HRPWM_2();
    fn HRPWM_3();
    fn CAN0_0();
    fn CAN0_1();
    fn CAN0_2();
    fn CAN0_3();
    fn CAN0_4();
    fn CAN0_5();
    fn CAN0_6();
    fn CAN0_7();
    fn USIC0_0();
    fn USIC0_1();
    fn USIC0_2();
    fn USIC0_3();
    fn USIC0_4();
    fn USIC0_5();
    fn USIC1_0();
    fn USIC1_1();
    fn USIC1_2();
    fn USIC1_3();
    fn USIC1_4();
    fn USIC1_5();
    fn LEDTS0_0();
    fn FCE0_0();
    fn GPDMA0_0();
    fn USB0_0();
}
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 108] = [
    Some(SCU_0),
    Some(ERU0_0),
    Some(ERU0_1),
    Some(ERU0_2),
    Some(ERU0_3),
    Some(ERU1_0),
    Some(ERU1_1),
    Some(ERU1_2),
    Some(ERU1_3),
    None,
    None,
    None,
    Some(PMU0_0),
    None,
    Some(VADC0_C0_0),
    Some(VADC0_C0_1),
    Some(VADC0_C0_2),
    Some(VADC0_C0_3),
    Some(VADC0_G0_0),
    Some(VADC0_G0_1),
    Some(VADC0_G0_2),
    Some(VADC0_G0_3),
    Some(VADC0_G1_0),
    Some(VADC0_G1_1),
    Some(VADC0_G1_2),
    Some(VADC0_G1_3),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(DAC0_0),
    Some(DAC0_1),
    Some(CCU40_0),
    Some(CCU40_1),
    Some(CCU40_2),
    Some(CCU40_3),
    Some(CCU41_0),
    Some(CCU41_1),
    Some(CCU41_2),
    Some(CCU41_3),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(CCU80_0),
    Some(CCU80_1),
    Some(CCU80_2),
    Some(CCU80_3),
    None,
    None,
    None,
    None,
    Some(POSIF0_0),
    Some(POSIF0_1),
    None,
    None,
    Some(HRPWM_0),
    Some(HRPWM_1),
    Some(HRPWM_2),
    Some(HRPWM_3),
    Some(CAN0_0),
    Some(CAN0_1),
    Some(CAN0_2),
    Some(CAN0_3),
    Some(CAN0_4),
    Some(CAN0_5),
    Some(CAN0_6),
    Some(CAN0_7),
    Some(USIC0_0),
    Some(USIC0_1),
    Some(USIC0_2),
    Some(USIC0_3),
    Some(USIC0_4),
    Some(USIC0_5),
    Some(USIC1_0),
    Some(USIC1_1),
    Some(USIC1_2),
    Some(USIC1_3),
    Some(USIC1_4),
    Some(USIC1_5),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(LEDTS0_0),
    None,
    Some(FCE0_0),
    Some(GPDMA0_0),
    None,
    Some(USB0_0),
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - System Control"]
    SCU_0,
    #[doc = "1 - External Request Unit 0"]
    ERU0_0,
    #[doc = "2 - External Request Unit 0"]
    ERU0_1,
    #[doc = "3 - External Request Unit 0"]
    ERU0_2,
    #[doc = "4 - External Request Unit 0"]
    ERU0_3,
    #[doc = "5 - External Request Unit 1"]
    ERU1_0,
    #[doc = "6 - External Request Unit 1"]
    ERU1_1,
    #[doc = "7 - External Request Unit 1"]
    ERU1_2,
    #[doc = "8 - External Request Unit 1"]
    ERU1_3,
    #[doc = "12 - Program Management Unit"]
    PMU0_0,
    #[doc = "14 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_0,
    #[doc = "15 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_1,
    #[doc = "16 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_2,
    #[doc = "17 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_3,
    #[doc = "18 - Analog to Digital Converter Group 0"]
    VADC0_G0_0,
    #[doc = "19 - Analog to Digital Converter Group 0"]
    VADC0_G0_1,
    #[doc = "20 - Analog to Digital Converter Group 0"]
    VADC0_G0_2,
    #[doc = "21 - Analog to Digital Converter Group 0"]
    VADC0_G0_3,
    #[doc = "22 - Analog to Digital Converter Group 1"]
    VADC0_G1_0,
    #[doc = "23 - Analog to Digital Converter Group 1"]
    VADC0_G1_1,
    #[doc = "24 - Analog to Digital Converter Group 1"]
    VADC0_G1_2,
    #[doc = "25 - Analog to Digital Converter Group 1"]
    VADC0_G1_3,
    #[doc = "42 - Digital to Analog Converter"]
    DAC0_0,
    #[doc = "43 - Digital to Analog Converter"]
    DAC0_1,
    #[doc = "44 - Capture Compare Unit 4 (Module 0)"]
    CCU40_0,
    #[doc = "45 - Capture Compare Unit 4 (Module 0)"]
    CCU40_1,
    #[doc = "46 - Capture Compare Unit 4 (Module 0)"]
    CCU40_2,
    #[doc = "47 - Capture Compare Unit 4 (Module 0)"]
    CCU40_3,
    #[doc = "48 - Capture Compare Unit 4 (Module 1)"]
    CCU41_0,
    #[doc = "49 - Capture Compare Unit 4 (Module 1)"]
    CCU41_1,
    #[doc = "50 - Capture Compare Unit 4 (Module 1)"]
    CCU41_2,
    #[doc = "51 - Capture Compare Unit 4 (Module 1)"]
    CCU41_3,
    #[doc = "60 - Capture Compare Unit 8 (Module 0)"]
    CCU80_0,
    #[doc = "61 - Capture Compare Unit 8 (Module 0)"]
    CCU80_1,
    #[doc = "62 - Capture Compare Unit 8 (Module 0)"]
    CCU80_2,
    #[doc = "63 - Capture Compare Unit 8 (Module 0)"]
    CCU80_3,
    #[doc = "68 - Position Interface (Module 0)"]
    POSIF0_0,
    #[doc = "69 - Position Interface (Module 0)"]
    POSIF0_1,
    #[doc = "72 - High Resolution Pulse Width Modulation (Module 0)"]
    HRPWM_0,
    #[doc = "73 - High Resolution Pulse Width Modulation (Module 0)"]
    HRPWM_1,
    #[doc = "74 - High Resolution Pulse Width Modulation (Module 0)"]
    HRPWM_2,
    #[doc = "75 - High Resolution Pulse Width Modulation (Module 0)"]
    HRPWM_3,
    #[doc = "76 - MultiCAN"]
    CAN0_0,
    #[doc = "77 - MultiCAN"]
    CAN0_1,
    #[doc = "78 - MultiCAN"]
    CAN0_2,
    #[doc = "79 - MultiCAN"]
    CAN0_3,
    #[doc = "80 - MultiCAN"]
    CAN0_4,
    #[doc = "81 - MultiCAN"]
    CAN0_5,
    #[doc = "82 - MultiCAN"]
    CAN0_6,
    #[doc = "83 - MultiCAN"]
    CAN0_7,
    #[doc = "84 - Universal Serial Interface Channel (Module 0)"]
    USIC0_0,
    #[doc = "85 - Universal Serial Interface Channel (Module 0)"]
    USIC0_1,
    #[doc = "86 - Universal Serial Interface Channel (Module 0)"]
    USIC0_2,
    #[doc = "87 - Universal Serial Interface Channel (Module 0)"]
    USIC0_3,
    #[doc = "88 - Universal Serial Interface Channel (Module 0)"]
    USIC0_4,
    #[doc = "89 - Universal Serial Interface Channel (Module 0)"]
    USIC0_5,
    #[doc = "90 - Universal Serial Interface Channel (Module 1)"]
    USIC1_0,
    #[doc = "91 - Universal Serial Interface Channel (Module 1)"]
    USIC1_1,
    #[doc = "92 - Universal Serial Interface Channel (Module 1)"]
    USIC1_2,
    #[doc = "93 - Universal Serial Interface Channel (Module 1)"]
    USIC1_3,
    #[doc = "94 - Universal Serial Interface Channel (Module 1)"]
    USIC1_4,
    #[doc = "95 - Universal Serial Interface Channel (Module 1)"]
    USIC1_5,
    #[doc = "102 - LED and Touch Sense Control Unit (Module 0)"]
    LEDTS0_0,
    #[doc = "104 - Flexible CRC Engine"]
    FCE0_0,
    #[doc = "105 - General Purpose DMA Unit 0"]
    GPDMA0_0,
    #[doc = "107 - Universal Serial Bus (Module 0)"]
    USB0_0,
}
unsafe impl Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::SCU_0 => 0,
            Interrupt::ERU0_0 => 1,
            Interrupt::ERU0_1 => 2,
            Interrupt::ERU0_2 => 3,
            Interrupt::ERU0_3 => 4,
            Interrupt::ERU1_0 => 5,
            Interrupt::ERU1_1 => 6,
            Interrupt::ERU1_2 => 7,
            Interrupt::ERU1_3 => 8,
            Interrupt::PMU0_0 => 12,
            Interrupt::VADC0_C0_0 => 14,
            Interrupt::VADC0_C0_1 => 15,
            Interrupt::VADC0_C0_2 => 16,
            Interrupt::VADC0_C0_3 => 17,
            Interrupt::VADC0_G0_0 => 18,
            Interrupt::VADC0_G0_1 => 19,
            Interrupt::VADC0_G0_2 => 20,
            Interrupt::VADC0_G0_3 => 21,
            Interrupt::VADC0_G1_0 => 22,
            Interrupt::VADC0_G1_1 => 23,
            Interrupt::VADC0_G1_2 => 24,
            Interrupt::VADC0_G1_3 => 25,
            Interrupt::DAC0_0 => 42,
            Interrupt::DAC0_1 => 43,
            Interrupt::CCU40_0 => 44,
            Interrupt::CCU40_1 => 45,
            Interrupt::CCU40_2 => 46,
            Interrupt::CCU40_3 => 47,
            Interrupt::CCU41_0 => 48,
            Interrupt::CCU41_1 => 49,
            Interrupt::CCU41_2 => 50,
            Interrupt::CCU41_3 => 51,
            Interrupt::CCU80_0 => 60,
            Interrupt::CCU80_1 => 61,
            Interrupt::CCU80_2 => 62,
            Interrupt::CCU80_3 => 63,
            Interrupt::POSIF0_0 => 68,
            Interrupt::POSIF0_1 => 69,
            Interrupt::HRPWM_0 => 72,
            Interrupt::HRPWM_1 => 73,
            Interrupt::HRPWM_2 => 74,
            Interrupt::HRPWM_3 => 75,
            Interrupt::CAN0_0 => 76,
            Interrupt::CAN0_1 => 77,
            Interrupt::CAN0_2 => 78,
            Interrupt::CAN0_3 => 79,
            Interrupt::CAN0_4 => 80,
            Interrupt::CAN0_5 => 81,
            Interrupt::CAN0_6 => 82,
            Interrupt::CAN0_7 => 83,
            Interrupt::USIC0_0 => 84,
            Interrupt::USIC0_1 => 85,
            Interrupt::USIC0_2 => 86,
            Interrupt::USIC0_3 => 87,
            Interrupt::USIC0_4 => 88,
            Interrupt::USIC0_5 => 89,
            Interrupt::USIC1_0 => 90,
            Interrupt::USIC1_1 => 91,
            Interrupt::USIC1_2 => 92,
            Interrupt::USIC1_3 => 93,
            Interrupt::USIC1_4 => 94,
            Interrupt::USIC1_5 => 95,
            Interrupt::LEDTS0_0 => 102,
            Interrupt::FCE0_0 => 104,
            Interrupt::GPDMA0_0 => 105,
            Interrupt::USB0_0 => 107,
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ident = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
