#[doc = "Register `transmit_config` reader"]
pub type R = crate::R<TRANSMIT_CONFIG_SPEC>;
#[doc = "Register `transmit_config` writer"]
pub type W = crate::W<TRANSMIT_CONFIG_SPEC>;
#[doc = "Field `function` reader - Enable transmit function"]
pub type FUNCTION_R = crate::BitReader<FUNCTION_A>;
#[doc = "Enable transmit function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUNCTION_A {
    #[doc = "1: Enable UART receive function signal"]
    ENABLE = 1,
    #[doc = "0: Disable UART receive function signal"]
    DISABLE = 0,
}
impl From<FUNCTION_A> for bool {
    #[inline(always)]
    fn from(variant: FUNCTION_A) -> Self {
        variant as u8 != 0
    }
}
impl FUNCTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FUNCTION_A {
        match self.bits {
            true => FUNCTION_A::ENABLE,
            false => FUNCTION_A::DISABLE,
        }
    }
    #[doc = "Enable UART receive function signal"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FUNCTION_A::ENABLE
    }
    #[doc = "Disable UART receive function signal"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FUNCTION_A::DISABLE
    }
}
#[doc = "Field `function` writer - Enable transmit function"]
pub type FUNCTION_W<'a, REG> = crate::BitWriter<'a, REG, FUNCTION_A>;
impl<'a, REG> FUNCTION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable UART receive function signal"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCTION_A::ENABLE)
    }
    #[doc = "Disable UART receive function signal"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCTION_A::DISABLE)
    }
}
#[doc = "Field `cts` reader - Enable Clear-to-Send flow control signal"]
pub type CTS_R = crate::BitReader<CTS_A>;
#[doc = "Enable Clear-to-Send flow control signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTS_A {
    #[doc = "1: Enable Clear-to-Send flow control signal"]
    ENABLE = 1,
    #[doc = "0: Disable Clear-to-Send flow control signal"]
    DISABLE = 0,
}
impl From<CTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_A) -> Self {
        variant as u8 != 0
    }
}
impl CTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTS_A {
        match self.bits {
            true => CTS_A::ENABLE,
            false => CTS_A::DISABLE,
        }
    }
    #[doc = "Enable Clear-to-Send flow control signal"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTS_A::ENABLE
    }
    #[doc = "Disable Clear-to-Send flow control signal"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTS_A::DISABLE
    }
}
#[doc = "Field `cts` writer - Enable Clear-to-Send flow control signal"]
pub type CTS_W<'a, REG> = crate::BitWriter<'a, REG, CTS_A>;
impl<'a, REG> CTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Clear-to-Send flow control signal"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CTS_A::ENABLE)
    }
    #[doc = "Disable Clear-to-Send flow control signal"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CTS_A::DISABLE)
    }
}
#[doc = "Field `freerun` reader - Enable freerun mode"]
pub type FREERUN_R = crate::BitReader<FREERUN_A>;
#[doc = "Enable freerun mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREERUN_A {
    #[doc = "1: Enable freerun mode"]
    ENABLE = 1,
    #[doc = "0: Disable freerun mode"]
    DISABLE = 0,
}
impl From<FREERUN_A> for bool {
    #[inline(always)]
    fn from(variant: FREERUN_A) -> Self {
        variant as u8 != 0
    }
}
impl FREERUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FREERUN_A {
        match self.bits {
            true => FREERUN_A::ENABLE,
            false => FREERUN_A::DISABLE,
        }
    }
    #[doc = "Enable freerun mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FREERUN_A::ENABLE
    }
    #[doc = "Disable freerun mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FREERUN_A::DISABLE
    }
}
#[doc = "Field `freerun` writer - Enable freerun mode"]
pub type FREERUN_W<'a, REG> = crate::BitWriter<'a, REG, FREERUN_A>;
impl<'a, REG> FREERUN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable freerun mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FREERUN_A::ENABLE)
    }
    #[doc = "Disable freerun mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FREERUN_A::DISABLE)
    }
}
#[doc = "Field `lin_transmit` reader - Local Interconnect Network protocol enable"]
pub type LIN_TRANSMIT_R = crate::BitReader<LIN_TRANSMIT_A>;
#[doc = "Local Interconnect Network protocol enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIN_TRANSMIT_A {
    #[doc = "1: Enable Local Interconnect Network protocol"]
    ENABLE = 1,
    #[doc = "0: Disable Local Interconnect Network protocol"]
    DISABLE = 0,
}
impl From<LIN_TRANSMIT_A> for bool {
    #[inline(always)]
    fn from(variant: LIN_TRANSMIT_A) -> Self {
        variant as u8 != 0
    }
}
impl LIN_TRANSMIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LIN_TRANSMIT_A {
        match self.bits {
            true => LIN_TRANSMIT_A::ENABLE,
            false => LIN_TRANSMIT_A::DISABLE,
        }
    }
    #[doc = "Enable Local Interconnect Network protocol"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LIN_TRANSMIT_A::ENABLE
    }
    #[doc = "Disable Local Interconnect Network protocol"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LIN_TRANSMIT_A::DISABLE
    }
}
#[doc = "Field `lin_transmit` writer - Local Interconnect Network protocol enable"]
pub type LIN_TRANSMIT_W<'a, REG> = crate::BitWriter<'a, REG, LIN_TRANSMIT_A>;
impl<'a, REG> LIN_TRANSMIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Local Interconnect Network protocol"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LIN_TRANSMIT_A::ENABLE)
    }
    #[doc = "Disable Local Interconnect Network protocol"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LIN_TRANSMIT_A::DISABLE)
    }
}
#[doc = "Field `parity_enable` reader - Enable transmit parity check"]
pub type PARITY_ENABLE_R = crate::BitReader<PARITY_ENABLE_A>;
#[doc = "Enable transmit parity check\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITY_ENABLE_A {
    #[doc = "1: Enable transmit parity check"]
    ENABLE = 1,
    #[doc = "0: Disable transmit parity check"]
    DISABLE = 0,
}
impl From<PARITY_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITY_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PARITY_ENABLE_A {
        match self.bits {
            true => PARITY_ENABLE_A::ENABLE,
            false => PARITY_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Enable transmit parity check"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PARITY_ENABLE_A::ENABLE
    }
    #[doc = "Disable transmit parity check"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PARITY_ENABLE_A::DISABLE
    }
}
#[doc = "Field `parity_enable` writer - Enable transmit parity check"]
pub type PARITY_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, PARITY_ENABLE_A>;
impl<'a, REG> PARITY_ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable transmit parity check"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_ENABLE_A::ENABLE)
    }
    #[doc = "Disable transmit parity check"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `parity_mode` reader - Select transmit parity mode if enabled"]
pub type PARITY_MODE_R = crate::BitReader<PARITY_MODE_A>;
#[doc = "Select transmit parity mode if enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITY_MODE_A {
    #[doc = "1: Odd parity if `parity_enable` is set"]
    ODD = 1,
    #[doc = "0: Even parity if `parity_enable` is set"]
    EVEN = 0,
}
impl From<PARITY_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITY_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PARITY_MODE_A {
        match self.bits {
            true => PARITY_MODE_A::ODD,
            false => PARITY_MODE_A::EVEN,
        }
    }
    #[doc = "Odd parity if `parity_enable` is set"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY_MODE_A::ODD
    }
    #[doc = "Even parity if `parity_enable` is set"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY_MODE_A::EVEN
    }
}
#[doc = "Field `parity_mode` writer - Select transmit parity mode if enabled"]
pub type PARITY_MODE_W<'a, REG> = crate::BitWriter<'a, REG, PARITY_MODE_A>;
impl<'a, REG> PARITY_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Odd parity if `parity_enable` is set"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_MODE_A::ODD)
    }
    #[doc = "Even parity if `parity_enable` is set"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PARITY_MODE_A::EVEN)
    }
}
#[doc = "Field `ir_transmit` reader - Enable IR transmit mode"]
pub type IR_TRANSMIT_R = crate::BitReader<IR_TRANSMIT_A>;
#[doc = "Enable IR transmit mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IR_TRANSMIT_A {
    #[doc = "1: Enable IR transmit mode"]
    ENABLE = 1,
    #[doc = "0: Disable IR transmit mode"]
    DISABLE = 0,
}
impl From<IR_TRANSMIT_A> for bool {
    #[inline(always)]
    fn from(variant: IR_TRANSMIT_A) -> Self {
        variant as u8 != 0
    }
}
impl IR_TRANSMIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IR_TRANSMIT_A {
        match self.bits {
            true => IR_TRANSMIT_A::ENABLE,
            false => IR_TRANSMIT_A::DISABLE,
        }
    }
    #[doc = "Enable IR transmit mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IR_TRANSMIT_A::ENABLE
    }
    #[doc = "Disable IR transmit mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IR_TRANSMIT_A::DISABLE
    }
}
#[doc = "Field `ir_transmit` writer - Enable IR transmit mode"]
pub type IR_TRANSMIT_W<'a, REG> = crate::BitWriter<'a, REG, IR_TRANSMIT_A>;
impl<'a, REG> IR_TRANSMIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable IR transmit mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IR_TRANSMIT_A::ENABLE)
    }
    #[doc = "Disable IR transmit mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IR_TRANSMIT_A::DISABLE)
    }
}
#[doc = "Field `ir_inverse` reader - Inverse transmit signal output in IR mode"]
pub type IR_INVERSE_R = crate::BitReader<IR_INVERSE_A>;
#[doc = "Inverse transmit signal output in IR mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IR_INVERSE_A {
    #[doc = "1: Inverse transmit input in IR mode"]
    INVERSE = 1,
    #[doc = "0: Don't inverse transmit input in IR mode"]
    NO_INVERSE = 0,
}
impl From<IR_INVERSE_A> for bool {
    #[inline(always)]
    fn from(variant: IR_INVERSE_A) -> Self {
        variant as u8 != 0
    }
}
impl IR_INVERSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IR_INVERSE_A {
        match self.bits {
            true => IR_INVERSE_A::INVERSE,
            false => IR_INVERSE_A::NO_INVERSE,
        }
    }
    #[doc = "Inverse transmit input in IR mode"]
    #[inline(always)]
    pub fn is_inverse(&self) -> bool {
        *self == IR_INVERSE_A::INVERSE
    }
    #[doc = "Don't inverse transmit input in IR mode"]
    #[inline(always)]
    pub fn is_no_inverse(&self) -> bool {
        *self == IR_INVERSE_A::NO_INVERSE
    }
}
#[doc = "Field `ir_inverse` writer - Inverse transmit signal output in IR mode"]
pub type IR_INVERSE_W<'a, REG> = crate::BitWriter<'a, REG, IR_INVERSE_A>;
impl<'a, REG> IR_INVERSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inverse transmit input in IR mode"]
    #[inline(always)]
    pub fn inverse(self) -> &'a mut crate::W<REG> {
        self.variant(IR_INVERSE_A::INVERSE)
    }
    #[doc = "Don't inverse transmit input in IR mode"]
    #[inline(always)]
    pub fn no_inverse(self) -> &'a mut crate::W<REG> {
        self.variant(IR_INVERSE_A::NO_INVERSE)
    }
}
#[doc = "Field `word_length` reader - Bit count for each transmit data word"]
pub type WORD_LENGTH_R = crate::FieldReader<WORD_LENGTH_A>;
#[doc = "Bit count for each transmit data word\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WORD_LENGTH_A {
    #[doc = "4: Each word includes 5 bits"]
    FIVE = 4,
    #[doc = "5: Each word includes 6 bits"]
    SIX = 5,
    #[doc = "6: Each word includes 7 bits"]
    SEVEN = 6,
    #[doc = "7: Each word includes 8 bits"]
    EIGHT = 7,
}
impl From<WORD_LENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WORD_LENGTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WORD_LENGTH_A {
    type Ux = u8;
}
impl WORD_LENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WORD_LENGTH_A> {
        match self.bits {
            4 => Some(WORD_LENGTH_A::FIVE),
            5 => Some(WORD_LENGTH_A::SIX),
            6 => Some(WORD_LENGTH_A::SEVEN),
            7 => Some(WORD_LENGTH_A::EIGHT),
            _ => None,
        }
    }
    #[doc = "Each word includes 5 bits"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == WORD_LENGTH_A::FIVE
    }
    #[doc = "Each word includes 6 bits"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == WORD_LENGTH_A::SIX
    }
    #[doc = "Each word includes 7 bits"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == WORD_LENGTH_A::SEVEN
    }
    #[doc = "Each word includes 8 bits"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == WORD_LENGTH_A::EIGHT
    }
}
#[doc = "Field `word_length` writer - Bit count for each transmit data word"]
pub type WORD_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WORD_LENGTH_A>;
impl<'a, REG> WORD_LENGTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Each word includes 5 bits"]
    #[inline(always)]
    pub fn five(self) -> &'a mut crate::W<REG> {
        self.variant(WORD_LENGTH_A::FIVE)
    }
    #[doc = "Each word includes 6 bits"]
    #[inline(always)]
    pub fn six(self) -> &'a mut crate::W<REG> {
        self.variant(WORD_LENGTH_A::SIX)
    }
    #[doc = "Each word includes 7 bits"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(WORD_LENGTH_A::SEVEN)
    }
    #[doc = "Each word includes 8 bits"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(WORD_LENGTH_A::EIGHT)
    }
}
#[doc = "Field `stop_bits` reader - Number of stop bits"]
pub type STOP_BITS_R = crate::FieldReader<STOP_BITS_A>;
#[doc = "Number of stop bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP_BITS_A {
    #[doc = "0: 0.5 stop bits"]
    ZERO_P_FIVE = 0,
    #[doc = "1: 1 stop bit"]
    ONE = 1,
    #[doc = "2: 1.5 stop bits"]
    ONE_P_FIVE = 2,
    #[doc = "3: 2 stop bits"]
    TWO = 3,
}
impl From<STOP_BITS_A> for u8 {
    #[inline(always)]
    fn from(variant: STOP_BITS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOP_BITS_A {
    type Ux = u8;
}
impl STOP_BITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP_BITS_A {
        match self.bits {
            0 => STOP_BITS_A::ZERO_P_FIVE,
            1 => STOP_BITS_A::ONE,
            2 => STOP_BITS_A::ONE_P_FIVE,
            3 => STOP_BITS_A::TWO,
            _ => unreachable!(),
        }
    }
    #[doc = "0.5 stop bits"]
    #[inline(always)]
    pub fn is_zero_p_five(&self) -> bool {
        *self == STOP_BITS_A::ZERO_P_FIVE
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == STOP_BITS_A::ONE
    }
    #[doc = "1.5 stop bits"]
    #[inline(always)]
    pub fn is_one_p_five(&self) -> bool {
        *self == STOP_BITS_A::ONE_P_FIVE
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == STOP_BITS_A::TWO
    }
}
#[doc = "Field `stop_bits` writer - Number of stop bits"]
pub type STOP_BITS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, STOP_BITS_A>;
impl<'a, REG> STOP_BITS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.5 stop bits"]
    #[inline(always)]
    pub fn zero_p_five(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_BITS_A::ZERO_P_FIVE)
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_BITS_A::ONE)
    }
    #[doc = "1.5 stop bits"]
    #[inline(always)]
    pub fn one_p_five(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_BITS_A::ONE_P_FIVE)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_BITS_A::TWO)
    }
}
#[doc = "Field `break_bits` reader - Number of break bits for LIN protocol"]
pub type BREAK_BITS_R = crate::FieldReader;
#[doc = "Field `break_bits` writer - Number of break bits for LIN protocol"]
pub type BREAK_BITS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `transfer_length` reader - Length of words per UART transmit transfer\n\n This field is ignored when `freerun` mode is enabled."]
pub type TRANSFER_LENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `transfer_length` writer - Length of words per UART transmit transfer\n\n This field is ignored when `freerun` mode is enabled."]
pub type TRANSFER_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable transmit function"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Clear-to-Send flow control signal"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable freerun mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Local Interconnect Network protocol enable"]
    #[inline(always)]
    pub fn lin_transmit(&self) -> LIN_TRANSMIT_R {
        LIN_TRANSMIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable transmit parity check"]
    #[inline(always)]
    pub fn parity_enable(&self) -> PARITY_ENABLE_R {
        PARITY_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select transmit parity mode if enabled"]
    #[inline(always)]
    pub fn parity_mode(&self) -> PARITY_MODE_R {
        PARITY_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable IR transmit mode"]
    #[inline(always)]
    pub fn ir_transmit(&self) -> IR_TRANSMIT_R {
        IR_TRANSMIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Inverse transmit signal output in IR mode"]
    #[inline(always)]
    pub fn ir_inverse(&self) -> IR_INVERSE_R {
        IR_INVERSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Bit count for each transmit data word"]
    #[inline(always)]
    pub fn word_length(&self) -> WORD_LENGTH_R {
        WORD_LENGTH_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Number of stop bits"]
    #[inline(always)]
    pub fn stop_bits(&self) -> STOP_BITS_R {
        STOP_BITS_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - Number of break bits for LIN protocol"]
    #[inline(always)]
    pub fn break_bits(&self) -> BREAK_BITS_R {
        BREAK_BITS_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Length of words per UART transmit transfer\n\n This field is ignored when `freerun` mode is enabled."]
    #[inline(always)]
    pub fn transfer_length(&self) -> TRANSFER_LENGTH_R {
        TRANSFER_LENGTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable transmit function"]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FUNCTION_W<TRANSMIT_CONFIG_SPEC> {
        FUNCTION_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Clear-to-Send flow control signal"]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CTS_W<TRANSMIT_CONFIG_SPEC> {
        CTS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable freerun mode"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FREERUN_W<TRANSMIT_CONFIG_SPEC> {
        FREERUN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Local Interconnect Network protocol enable"]
    #[inline(always)]
    #[must_use]
    pub fn lin_transmit(&mut self) -> LIN_TRANSMIT_W<TRANSMIT_CONFIG_SPEC> {
        LIN_TRANSMIT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable transmit parity check"]
    #[inline(always)]
    #[must_use]
    pub fn parity_enable(&mut self) -> PARITY_ENABLE_W<TRANSMIT_CONFIG_SPEC> {
        PARITY_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Select transmit parity mode if enabled"]
    #[inline(always)]
    #[must_use]
    pub fn parity_mode(&mut self) -> PARITY_MODE_W<TRANSMIT_CONFIG_SPEC> {
        PARITY_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable IR transmit mode"]
    #[inline(always)]
    #[must_use]
    pub fn ir_transmit(&mut self) -> IR_TRANSMIT_W<TRANSMIT_CONFIG_SPEC> {
        IR_TRANSMIT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Inverse transmit signal output in IR mode"]
    #[inline(always)]
    #[must_use]
    pub fn ir_inverse(&mut self) -> IR_INVERSE_W<TRANSMIT_CONFIG_SPEC> {
        IR_INVERSE_W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Bit count for each transmit data word"]
    #[inline(always)]
    #[must_use]
    pub fn word_length(&mut self) -> WORD_LENGTH_W<TRANSMIT_CONFIG_SPEC> {
        WORD_LENGTH_W::new(self, 8)
    }
    #[doc = "Bits 11:12 - Number of stop bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop_bits(&mut self) -> STOP_BITS_W<TRANSMIT_CONFIG_SPEC> {
        STOP_BITS_W::new(self, 11)
    }
    #[doc = "Bits 13:15 - Number of break bits for LIN protocol"]
    #[inline(always)]
    #[must_use]
    pub fn break_bits(&mut self) -> BREAK_BITS_W<TRANSMIT_CONFIG_SPEC> {
        BREAK_BITS_W::new(self, 13)
    }
    #[doc = "Bits 16:31 - Length of words per UART transmit transfer\n\n This field is ignored when `freerun` mode is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn transfer_length(&mut self) -> TRANSFER_LENGTH_W<TRANSMIT_CONFIG_SPEC> {
        TRANSFER_LENGTH_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRANSMIT_CONFIG_SPEC;
impl crate::RegisterSpec for TRANSMIT_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transmit_config::R`](R) reader structure"]
impl crate::Readable for TRANSMIT_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`transmit_config::W`](W) writer structure"]
impl crate::Writable for TRANSMIT_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets transmit_config to value 0x8f00"]
impl crate::Resettable for TRANSMIT_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x8f00;
}
