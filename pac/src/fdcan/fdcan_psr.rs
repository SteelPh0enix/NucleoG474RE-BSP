#[doc = "Register `FDCAN_PSR` reader"]
pub struct R(crate::R<FDCAN_PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_PSR` writer"]
pub struct W(crate::W<FDCAN_PSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FDCAN_PSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_PSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEC` reader - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
pub type LEC_R = crate::FieldReader<LEC_A>;
#[doc = "Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEC_A {
    #[doc = "0: No Error: No error occurred since LEC has been reset by successful reception or transmission."]
    B_0X0 = 0,
    #[doc = "1: Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    B_0X1 = 1,
    #[doc = "2: Form Error: A fixed format part of a received frame has the wrong format."]
    B_0X2 = 2,
    #[doc = "3: AckError: The message transmitted by the FDCAN was not acknowledged by another node."]
    B_0X3 = 3,
    #[doc = "4: Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    B_0X4 = 4,
    #[doc = "5: Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    B_0X5 = 5,
    #[doc = "6: CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data."]
    B_0X6 = 6,
    #[doc = "7: NoChange: Any read access to the Protocol status register re-initializes the LEC to 7. When the LEC shows the value 7, no CAN bus event was detected since the last CPU read access to the Protocol status register."]
    B_0X7 = 7,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LEC_A {
    type Ux = u8;
}
impl LEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::B_0X0,
            1 => LEC_A::B_0X1,
            2 => LEC_A::B_0X2,
            3 => LEC_A::B_0X3,
            4 => LEC_A::B_0X4,
            5 => LEC_A::B_0X5,
            6 => LEC_A::B_0X6,
            7 => LEC_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LEC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LEC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == LEC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LEC_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == LEC_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == LEC_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == LEC_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == LEC_A::B_0X7
    }
}
#[doc = "Field `LEC` writer - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
pub type LEC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, FDCAN_PSR_SPEC, 3, O, LEC_A>;
impl<'a, const O: u8> LEC_W<'a, O> {
    #[doc = "No Error: No error occurred since LEC has been reset by successful reception or transmission."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LEC_A::B_0X0)
    }
    #[doc = "Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LEC_A::B_0X1)
    }
    #[doc = "Form Error: A fixed format part of a received frame has the wrong format."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(LEC_A::B_0X2)
    }
    #[doc = "AckError: The message transmitted by the FDCAN was not acknowledged by another node."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LEC_A::B_0X3)
    }
    #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(LEC_A::B_0X4)
    }
    #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(LEC_A::B_0X5)
    }
    #[doc = "CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(LEC_A::B_0X6)
    }
    #[doc = "NoChange: Any read access to the Protocol status register re-initializes the LEC to 7. When the LEC shows the value 7, no CAN bus event was detected since the last CPU read access to the Protocol status register."]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(LEC_A::B_0X7)
    }
}
#[doc = "Field `ACT` reader - Activity Monitors the modules CAN communication state."]
pub type ACT_R = crate::FieldReader<ACT_A>;
#[doc = "Activity Monitors the modules CAN communication state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACT_A {
    #[doc = "0: Synchronizing: node is synchronizing on CAN communication."]
    B_0X0 = 0,
    #[doc = "1: Idle: node is neither receiver nor transmitter."]
    B_0X1 = 1,
    #[doc = "2: Receiver: node is operating as receiver."]
    B_0X2 = 2,
    #[doc = "3: Transmitter: node is operating as transmitter."]
    B_0X3 = 3,
}
impl From<ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACT_A {
    type Ux = u8;
}
impl ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACT_A {
        match self.bits {
            0 => ACT_A::B_0X0,
            1 => ACT_A::B_0X1,
            2 => ACT_A::B_0X2,
            3 => ACT_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ACT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ACT_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ACT_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ACT_A::B_0X3
    }
}
#[doc = "Field `EP` reader - Error passive"]
pub type EP_R = crate::BitReader<EP_A>;
#[doc = "Error passive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP_A {
    #[doc = "0: The FDCAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected."]
    B_0X0 = 0,
    #[doc = "1: The FDCAN is in the Error_Passive state."]
    B_0X1 = 1,
}
impl From<EP_A> for bool {
    #[inline(always)]
    fn from(variant: EP_A) -> Self {
        variant as u8 != 0
    }
}
impl EP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP_A {
        match self.bits {
            false => EP_A::B_0X0,
            true => EP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EP_A::B_0X1
    }
}
#[doc = "Field `EW` reader - Warning Sstatus"]
pub type EW_R = crate::BitReader<EW_A>;
#[doc = "Warning Sstatus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EW_A {
    #[doc = "0: Both error counters are below the Error_Warning limit of 96."]
    B_0X0 = 0,
    #[doc = "1: At least one of error counter has reached the Error_Warning limit of 96."]
    B_0X1 = 1,
}
impl From<EW_A> for bool {
    #[inline(always)]
    fn from(variant: EW_A) -> Self {
        variant as u8 != 0
    }
}
impl EW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_A {
        match self.bits {
            false => EW_A::B_0X0,
            true => EW_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EW_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EW_A::B_0X1
    }
}
#[doc = "Field `BO` reader - Bus_Off status"]
pub type BO_R = crate::BitReader<BO_A>;
#[doc = "Bus_Off status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BO_A {
    #[doc = "0: The FDCAN is not Bus_Off."]
    B_0X0 = 0,
    #[doc = "1: The FDCAN is in Bus_Off state."]
    B_0X1 = 1,
}
impl From<BO_A> for bool {
    #[inline(always)]
    fn from(variant: BO_A) -> Self {
        variant as u8 != 0
    }
}
impl BO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BO_A {
        match self.bits {
            false => BO_A::B_0X0,
            true => BO_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BO_A::B_0X1
    }
}
#[doc = "Field `DLEC` reader - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
pub type DLEC_R = crate::FieldReader;
#[doc = "Field `DLEC` writer - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
pub type DLEC_W<'a, const O: u8> = crate::FieldWriter<'a, FDCAN_PSR_SPEC, 3, O>;
#[doc = "Field `RESI` reader - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
pub type RESI_R = crate::BitReader<RESI_A>;
#[doc = "ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESI_A {
    #[doc = "0: Last received FDCAN message did not have its ESI flag set."]
    B_0X0 = 0,
    #[doc = "1: Last received FDCAN message had its ESI flag set."]
    B_0X1 = 1,
}
impl From<RESI_A> for bool {
    #[inline(always)]
    fn from(variant: RESI_A) -> Self {
        variant as u8 != 0
    }
}
impl RESI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESI_A {
        match self.bits {
            false => RESI_A::B_0X0,
            true => RESI_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RESI_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RESI_A::B_0X1
    }
}
#[doc = "Field `RESI` writer - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
pub type RESI_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_PSR_SPEC, O, RESI_A>;
impl<'a, const O: u8> RESI_W<'a, O> {
    #[doc = "Last received FDCAN message did not have its ESI flag set."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RESI_A::B_0X0)
    }
    #[doc = "Last received FDCAN message had its ESI flag set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RESI_A::B_0X1)
    }
}
#[doc = "Field `RBRS` reader - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
pub type RBRS_R = crate::BitReader<RBRS_A>;
#[doc = "BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBRS_A {
    #[doc = "0: Last received FDCAN message did not have its BRS flag set."]
    B_0X0 = 0,
    #[doc = "1: Last received FDCAN message had its BRS flag set."]
    B_0X1 = 1,
}
impl From<RBRS_A> for bool {
    #[inline(always)]
    fn from(variant: RBRS_A) -> Self {
        variant as u8 != 0
    }
}
impl RBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBRS_A {
        match self.bits {
            false => RBRS_A::B_0X0,
            true => RBRS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RBRS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RBRS_A::B_0X1
    }
}
#[doc = "Field `RBRS` writer - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
pub type RBRS_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_PSR_SPEC, O, RBRS_A>;
impl<'a, const O: u8> RBRS_W<'a, O> {
    #[doc = "Last received FDCAN message did not have its BRS flag set."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RBRS_A::B_0X0)
    }
    #[doc = "Last received FDCAN message had its BRS flag set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RBRS_A::B_0X1)
    }
}
#[doc = "Field `REDL` reader - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
pub type REDL_R = crate::BitReader<REDL_A>;
#[doc = "Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REDL_A {
    #[doc = "0: Since this bit was reset by the CPU, no FDCAN message has been received."]
    B_0X0 = 0,
    #[doc = "1: Message in FDCAN format with EDL flag set has been received."]
    B_0X1 = 1,
}
impl From<REDL_A> for bool {
    #[inline(always)]
    fn from(variant: REDL_A) -> Self {
        variant as u8 != 0
    }
}
impl REDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REDL_A {
        match self.bits {
            false => REDL_A::B_0X0,
            true => REDL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REDL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REDL_A::B_0X1
    }
}
#[doc = "Field `REDL` writer - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
pub type REDL_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_PSR_SPEC, O, REDL_A>;
impl<'a, const O: u8> REDL_W<'a, O> {
    #[doc = "Since this bit was reset by the CPU, no FDCAN message has been received."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(REDL_A::B_0X0)
    }
    #[doc = "Message in FDCAN format with EDL flag set has been received."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(REDL_A::B_0X1)
    }
}
#[doc = "Field `PXE` reader - Protocol exception event"]
pub type PXE_R = crate::BitReader<PXE_A>;
#[doc = "Protocol exception event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PXE_A {
    #[doc = "0: No protocol exception event occurred since last read access"]
    B_0X0 = 0,
    #[doc = "1: Protocol exception event occurred"]
    B_0X1 = 1,
}
impl From<PXE_A> for bool {
    #[inline(always)]
    fn from(variant: PXE_A) -> Self {
        variant as u8 != 0
    }
}
impl PXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PXE_A {
        match self.bits {
            false => PXE_A::B_0X0,
            true => PXE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PXE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PXE_A::B_0X1
    }
}
#[doc = "Field `PXE` writer - Protocol exception event"]
pub type PXE_W<'a, const O: u8> = crate::BitWriter<'a, FDCAN_PSR_SPEC, O, PXE_A>;
impl<'a, const O: u8> PXE_W<'a, O> {
    #[doc = "No protocol exception event occurred since last read access"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PXE_A::B_0X0)
    }
    #[doc = "Protocol exception event occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PXE_A::B_0X1)
    }
}
#[doc = "Field `TDCV` reader - Transmitter delay compensation value Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
pub type TDCV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Activity Monitors the modules CAN communication state."]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Error passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Sstatus"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus_Off status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol exception event"]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter delay compensation value Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<0> {
        LEC_W::new(self)
    }
    #[doc = "Bits 8:10 - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    #[must_use]
    pub fn dlec(&mut self) -> DLEC_W<8> {
        DLEC_W::new(self)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    #[must_use]
    pub fn resi(&mut self) -> RESI_W<11> {
        RESI_W::new(self)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    #[must_use]
    pub fn rbrs(&mut self) -> RBRS_W<12> {
        RBRS_W::new(self)
    }
    #[doc = "Bit 13 - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    #[must_use]
    pub fn redl(&mut self) -> REDL_W<13> {
        REDL_W::new(self)
    }
    #[doc = "Bit 14 - Protocol exception event"]
    #[inline(always)]
    #[must_use]
    pub fn pxe(&mut self) -> PXE_W<14> {
        PXE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN protocol status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_psr](index.html) module"]
pub struct FDCAN_PSR_SPEC;
impl crate::RegisterSpec for FDCAN_PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_psr::R](R) reader structure"]
impl crate::Readable for FDCAN_PSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_psr::W](W) writer structure"]
impl crate::Writable for FDCAN_PSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_PSR to value 0x0707"]
impl crate::Resettable for FDCAN_PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0707;
}
