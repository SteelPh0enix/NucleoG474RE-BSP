#[doc = "Register `RCC_CCIPR` reader"]
pub struct R(crate::R<RCC_CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CCIPR` writer"]
pub struct W(crate::W<RCC_CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CCIPR_SPEC>;
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
impl From<crate::W<RCC_CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART1SEL` reader - USART1 clock source selection This bit is set and cleared by software to select the USART1 clock source."]
pub type USART1SEL_R = crate::FieldReader<USART1SEL_A>;
#[doc = "USART1 clock source selection This bit is set and cleared by software to select the USART1 clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL_A {
    #[doc = "0: PCLK selected as USART1 clock"]
    B_0X0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as USART1 clock"]
    B_0X1 = 1,
    #[doc = "2: HSI16 clock selected as USART1 clock"]
    B_0X2 = 2,
    #[doc = "3: LSE clock selected as USART1 clock"]
    B_0X3 = 3,
}
impl From<USART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART1SEL_A {
    type Ux = u8;
}
impl USART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1SEL_A {
        match self.bits {
            0 => USART1SEL_A::B_0X0,
            1 => USART1SEL_A::B_0X1,
            2 => USART1SEL_A::B_0X2,
            3 => USART1SEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART1SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART1SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == USART1SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == USART1SEL_A::B_0X3
    }
}
#[doc = "Field `USART1SEL` writer - USART1 clock source selection This bit is set and cleared by software to select the USART1 clock source."]
pub type USART1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, RCC_CCIPR_SPEC, 2, O, USART1SEL_A>;
impl<'a, const O: u8> USART1SEL_W<'a, O> {
    #[doc = "PCLK selected as USART1 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART1SEL_A::B_0X0)
    }
    #[doc = "System clock (SYSCLK) selected as USART1 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART1SEL_A::B_0X1)
    }
    #[doc = "HSI16 clock selected as USART1 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(USART1SEL_A::B_0X2)
    }
    #[doc = "LSE clock selected as USART1 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(USART1SEL_A::B_0X3)
    }
}
#[doc = "Field `USART2SEL` reader - USART2 clock source selection This bit is set and cleared by software to select the USART2 clock source."]
pub type USART2SEL_R = crate::FieldReader<USART2SEL_A>;
#[doc = "USART2 clock source selection This bit is set and cleared by software to select the USART2 clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART2SEL_A {
    #[doc = "0: PCLK selected as USART2 clock"]
    B_0X0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as USART2 clock"]
    B_0X1 = 1,
    #[doc = "2: HSI16 clock selected as USART2 clock"]
    B_0X2 = 2,
    #[doc = "3: LSE clock selected as USART2 clock"]
    B_0X3 = 3,
}
impl From<USART2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART2SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART2SEL_A {
    type Ux = u8;
}
impl USART2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART2SEL_A {
        match self.bits {
            0 => USART2SEL_A::B_0X0,
            1 => USART2SEL_A::B_0X1,
            2 => USART2SEL_A::B_0X2,
            3 => USART2SEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART2SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART2SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == USART2SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == USART2SEL_A::B_0X3
    }
}
#[doc = "Field `USART2SEL` writer - USART2 clock source selection This bit is set and cleared by software to select the USART2 clock source."]
pub type USART2SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, RCC_CCIPR_SPEC, 2, O, USART2SEL_A>;
impl<'a, const O: u8> USART2SEL_W<'a, O> {
    #[doc = "PCLK selected as USART2 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART2SEL_A::B_0X0)
    }
    #[doc = "System clock (SYSCLK) selected as USART2 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART2SEL_A::B_0X1)
    }
    #[doc = "HSI16 clock selected as USART2 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(USART2SEL_A::B_0X2)
    }
    #[doc = "LSE clock selected as USART2 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(USART2SEL_A::B_0X3)
    }
}
#[doc = "Field `USART3SEL` reader - USART3 clock source selection This bit is set and cleared by software to select the USART3 clock source."]
pub type USART3SEL_R = crate::FieldReader<USART3SEL_A>;
#[doc = "USART3 clock source selection This bit is set and cleared by software to select the USART3 clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART3SEL_A {
    #[doc = "0: PCLK selected as USART3 clock"]
    B_0X0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as USART3 clock"]
    B_0X1 = 1,
    #[doc = "2: HSI16 clock selected as USART3 clock"]
    B_0X2 = 2,
    #[doc = "3: LSE clock selected as USART3 clock"]
    B_0X3 = 3,
}
impl From<USART3SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART3SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART3SEL_A {
    type Ux = u8;
}
impl USART3SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART3SEL_A {
        match self.bits {
            0 => USART3SEL_A::B_0X0,
            1 => USART3SEL_A::B_0X1,
            2 => USART3SEL_A::B_0X2,
            3 => USART3SEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART3SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART3SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == USART3SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == USART3SEL_A::B_0X3
    }
}
#[doc = "Field `USART3SEL` writer - USART3 clock source selection This bit is set and cleared by software to select the USART3 clock source."]
pub type USART3SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, RCC_CCIPR_SPEC, 2, O, USART3SEL_A>;
impl<'a, const O: u8> USART3SEL_W<'a, O> {
    #[doc = "PCLK selected as USART3 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART3SEL_A::B_0X0)
    }
    #[doc = "System clock (SYSCLK) selected as USART3 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART3SEL_A::B_0X1)
    }
    #[doc = "HSI16 clock selected as USART3 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(USART3SEL_A::B_0X2)
    }
    #[doc = "LSE clock selected as USART3 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(USART3SEL_A::B_0X3)
    }
}
#[doc = "Field `UART4SEL` reader - UART4 clock source selection This bit is set and cleared by software to select the UART4 clock source."]
pub type UART4SEL_R = crate::FieldReader<UART4SEL_A>;
#[doc = "UART4 clock source selection This bit is set and cleared by software to select the UART4 clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART4SEL_A {
    #[doc = "0: PCLK selected as UART4 clock"]
    B_0X0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as UART4 clock"]
    B_0X1 = 1,
    #[doc = "2: HSI16 clock selected as UART4 clock"]
    B_0X2 = 2,
    #[doc = "3: LSE clock selected as UART4 clock"]
    B_0X3 = 3,
}
impl From<UART4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UART4SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UART4SEL_A {
    type Ux = u8;
}
impl UART4SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART4SEL_A {
        match self.bits {
            0 => UART4SEL_A::B_0X0,
            1 => UART4SEL_A::B_0X1,
            2 => UART4SEL_A::B_0X2,
            3 => UART4SEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UART4SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UART4SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == UART4SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == UART4SEL_A::B_0X3
    }
}
#[doc = "Field `UART4SEL` writer - UART4 clock source selection This bit is set and cleared by software to select the UART4 clock source."]
pub type UART4SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, RCC_CCIPR_SPEC, 2, O, UART4SEL_A>;
impl<'a, const O: u8> UART4SEL_W<'a, O> {
    #[doc = "PCLK selected as UART4 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UART4SEL_A::B_0X0)
    }
    #[doc = "System clock (SYSCLK) selected as UART4 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UART4SEL_A::B_0X1)
    }
    #[doc = "HSI16 clock selected as UART4 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(UART4SEL_A::B_0X2)
    }
    #[doc = "LSE clock selected as UART4 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(UART4SEL_A::B_0X3)
    }
}
#[doc = "Field `UART5SEL` reader - UART5 clock source selection These bits are set and cleared by software to select the UART5 clock source."]
pub type UART5SEL_R = crate::FieldReader<UART5SEL_A>;
#[doc = "UART5 clock source selection These bits are set and cleared by software to select the UART5 clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART5SEL_A {
    #[doc = "0: PCLK selected as UART5 clock"]
    B_0X0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as UART5 clock"]
    B_0X1 = 1,
    #[doc = "2: HSI16 clock selected as UART5 clock"]
    B_0X2 = 2,
    #[doc = "3: LSE clock selected as UART5 clock"]
    B_0X3 = 3,
}
impl From<UART5SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UART5SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UART5SEL_A {
    type Ux = u8;
}
impl UART5SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART5SEL_A {
        match self.bits {
            0 => UART5SEL_A::B_0X0,
            1 => UART5SEL_A::B_0X1,
            2 => UART5SEL_A::B_0X2,
            3 => UART5SEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UART5SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UART5SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == UART5SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == UART5SEL_A::B_0X3
    }
}
#[doc = "Field `UART5SEL` writer - UART5 clock source selection These bits are set and cleared by software to select the UART5 clock source."]
pub type UART5SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, RCC_CCIPR_SPEC, 2, O, UART5SEL_A>;
impl<'a, const O: u8> UART5SEL_W<'a, O> {
    #[doc = "PCLK selected as UART5 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UART5SEL_A::B_0X0)
    }
    #[doc = "System clock (SYSCLK) selected as UART5 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UART5SEL_A::B_0X1)
    }
    #[doc = "HSI16 clock selected as UART5 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(UART5SEL_A::B_0X2)
    }
    #[doc = "LSE clock selected as UART5 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(UART5SEL_A::B_0X3)
    }
}
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection These bits are set and cleared by software to select the LPUART1 clock source."]
pub type LPUART1SEL_R = crate::FieldReader<LPUART1SEL_A>;
#[doc = "LPUART1 clock source selection These bits are set and cleared by software to select the LPUART1 clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPUART1SEL_A {
    #[doc = "0: PCLK selected as LPUART1 clock"]
    B_0X0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as LPUART1 clock"]
    B_0X1 = 1,
    #[doc = "2: HSI16 clock selected as LPUART1 clock"]
    B_0X2 = 2,
    #[doc = "3: LSE clock selected as LPUART1 clock"]
    B_0X3 = 3,
}
impl From<LPUART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPUART1SEL_A {
    type Ux = u8;
}
impl LPUART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1SEL_A {
        match self.bits {
            0 => LPUART1SEL_A::B_0X0,
            1 => LPUART1SEL_A::B_0X1,
            2 => LPUART1SEL_A::B_0X2,
            3 => LPUART1SEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPUART1SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPUART1SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == LPUART1SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LPUART1SEL_A::B_0X3
    }
}
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection These bits are set and cleared by software to select the LPUART1 clock source."]
pub type LPUART1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, RCC_CCIPR_SPEC, 2, O, LPUART1SEL_A>;
impl<'a, const O: u8> LPUART1SEL_W<'a, O> {
    #[doc = "PCLK selected as LPUART1 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::B_0X0)
    }
    #[doc = "System clock (SYSCLK) selected as LPUART1 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::B_0X1)
    }
    #[doc = "HSI16 clock selected as LPUART1 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::B_0X2)
    }
    #[doc = "LSE clock selected as LPUART1 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::B_0X3)
    }
}
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection These bits are set and cleared by software to select the I2C1 clock source."]
pub type I2C1SEL_R = crate::FieldReader<I2C1SEL_A>;
#[doc = "I2C1 clock source selection These bits are set and cleared by software to select the I2C1 clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL_A {
    #[doc = "0: PCLK selected as I2C1 clock"]
    B_0X0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as I2C1 clock"]
    B_0X1 = 1,
    #[doc = "2: HSI16 clock selected as I2C1 clock"]
    B_0X2 = 2,
}
impl From<I2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C1SEL_A {
    type Ux = u8;
}
impl I2C1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1SEL_A> {
        match self.bits {
            0 => Some(I2C1SEL_A::B_0X0),
            1 => Some(I2C1SEL_A::B_0X1),
            2 => Some(I2C1SEL_A::B_0X2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C1SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C1SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2C1SEL_A::B_0X2
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection These bits are set and cleared by software to select the I2C1 clock source."]
pub type I2C1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CCIPR_SPEC, 2, O, I2C1SEL_A>;
impl<'a, const O: u8> I2C1SEL_W<'a, O> {
    #[doc = "PCLK selected as I2C1 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C1SEL_A::B_0X0)
    }
    #[doc = "System clock (SYSCLK) selected as I2C1 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C1SEL_A::B_0X1)
    }
    #[doc = "HSI16 clock selected as I2C1 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(I2C1SEL_A::B_0X2)
    }
}
#[doc = "Field `I2C2SEL` reader - I2C2 clock source selection These bits are set and cleared by software to select the I2C2 clock source."]
pub type I2C2SEL_R = crate::FieldReader<I2C2SEL_A>;
#[doc = "I2C2 clock source selection These bits are set and cleared by software to select the I2C2 clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C2SEL_A {
    #[doc = "0: PCLK selected as I2C2 clock"]
    B_0X0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as I2C2 clock"]
    B_0X1 = 1,
    #[doc = "2: HSI16 clock selected as I2C2 clock"]
    B_0X2 = 2,
}
impl From<I2C2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C2SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C2SEL_A {
    type Ux = u8;
}
impl I2C2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C2SEL_A> {
        match self.bits {
            0 => Some(I2C2SEL_A::B_0X0),
            1 => Some(I2C2SEL_A::B_0X1),
            2 => Some(I2C2SEL_A::B_0X2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C2SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C2SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2C2SEL_A::B_0X2
    }
}
#[doc = "Field `I2C2SEL` writer - I2C2 clock source selection These bits are set and cleared by software to select the I2C2 clock source."]
pub type I2C2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CCIPR_SPEC, 2, O, I2C2SEL_A>;
impl<'a, const O: u8> I2C2SEL_W<'a, O> {
    #[doc = "PCLK selected as I2C2 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C2SEL_A::B_0X0)
    }
    #[doc = "System clock (SYSCLK) selected as I2C2 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C2SEL_A::B_0X1)
    }
    #[doc = "HSI16 clock selected as I2C2 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(I2C2SEL_A::B_0X2)
    }
}
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection These bits are set and cleared by software to select the I2C3 clock source."]
pub type I2C3SEL_R = crate::FieldReader<I2C3SEL_A>;
#[doc = "I2C3 clock source selection These bits are set and cleared by software to select the I2C3 clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C3SEL_A {
    #[doc = "0: PCLK selected as I2C3 clock"]
    B_0X0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as I2C3 clock"]
    B_0X1 = 1,
    #[doc = "2: HSI16 clock selected as I2C3 clock"]
    B_0X2 = 2,
}
impl From<I2C3SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C3SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C3SEL_A {
    type Ux = u8;
}
impl I2C3SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C3SEL_A> {
        match self.bits {
            0 => Some(I2C3SEL_A::B_0X0),
            1 => Some(I2C3SEL_A::B_0X1),
            2 => Some(I2C3SEL_A::B_0X2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C3SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C3SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2C3SEL_A::B_0X2
    }
}
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection These bits are set and cleared by software to select the I2C3 clock source."]
pub type I2C3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CCIPR_SPEC, 2, O, I2C3SEL_A>;
impl<'a, const O: u8> I2C3SEL_W<'a, O> {
    #[doc = "PCLK selected as I2C3 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C3SEL_A::B_0X0)
    }
    #[doc = "System clock (SYSCLK) selected as I2C3 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C3SEL_A::B_0X1)
    }
    #[doc = "HSI16 clock selected as I2C3 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(I2C3SEL_A::B_0X2)
    }
}
#[doc = "Field `LPTIM1SEL` reader - Low power timer 1 clock source selection These bits are set and cleared by software to select the LPTIM1 clock source."]
pub type LPTIM1SEL_R = crate::FieldReader<LPTIM1SEL_A>;
#[doc = "Low power timer 1 clock source selection These bits are set and cleared by software to select the LPTIM1 clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    #[doc = "0: PCLK selected as LPTIM1 clock"]
    B_0X0 = 0,
    #[doc = "1: LSI clock selected as LPTIM1 clock"]
    B_0X1 = 1,
    #[doc = "2: HSI16 clock selected as LPTIM1 clock"]
    B_0X2 = 2,
    #[doc = "3: LSE clock selected as LPTIM1 clock"]
    B_0X3 = 3,
}
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM1SEL_A {
    type Ux = u8;
}
impl LPTIM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SEL_A {
        match self.bits {
            0 => LPTIM1SEL_A::B_0X0,
            1 => LPTIM1SEL_A::B_0X1,
            2 => LPTIM1SEL_A::B_0X2,
            3 => LPTIM1SEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM1SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM1SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == LPTIM1SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LPTIM1SEL_A::B_0X3
    }
}
#[doc = "Field `LPTIM1SEL` writer - Low power timer 1 clock source selection These bits are set and cleared by software to select the LPTIM1 clock source."]
pub type LPTIM1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, RCC_CCIPR_SPEC, 2, O, LPTIM1SEL_A>;
impl<'a, const O: u8> LPTIM1SEL_W<'a, O> {
    #[doc = "PCLK selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::B_0X0)
    }
    #[doc = "LSI clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::B_0X1)
    }
    #[doc = "HSI16 clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::B_0X2)
    }
    #[doc = "LSE clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::B_0X3)
    }
}
#[doc = "Field `SAI1SEL` reader - clock source selection These bits are set and cleared by software to select the SAI clock source."]
pub type SAI1SEL_R = crate::FieldReader<SAI1SEL_A>;
#[doc = "clock source selection These bits are set and cleared by software to select the SAI clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL_A {
    #[doc = "0: System clock selected as SAI clock"]
    B_0X0 = 0,
    #[doc = "1: PLL Q clock selected as SAI clock"]
    B_0X1 = 1,
    #[doc = "2: Clock provided on I2S_CKIN pin selected as SAI clock"]
    B_0X2 = 2,
    #[doc = "3: HSI16 clock selected as SAI clock"]
    B_0X3 = 3,
}
impl From<SAI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAI1SEL_A {
    type Ux = u8;
}
impl SAI1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1SEL_A {
        match self.bits {
            0 => SAI1SEL_A::B_0X0,
            1 => SAI1SEL_A::B_0X1,
            2 => SAI1SEL_A::B_0X2,
            3 => SAI1SEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI1SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI1SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SAI1SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SAI1SEL_A::B_0X3
    }
}
#[doc = "Field `SAI1SEL` writer - clock source selection These bits are set and cleared by software to select the SAI clock source."]
pub type SAI1SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, RCC_CCIPR_SPEC, 2, O, SAI1SEL_A>;
impl<'a, const O: u8> SAI1SEL_W<'a, O> {
    #[doc = "System clock selected as SAI clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI1SEL_A::B_0X0)
    }
    #[doc = "PLL Q clock selected as SAI clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI1SEL_A::B_0X1)
    }
    #[doc = "Clock provided on I2S_CKIN pin selected as SAI clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SAI1SEL_A::B_0X2)
    }
    #[doc = "HSI16 clock selected as SAI clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SAI1SEL_A::B_0X3)
    }
}
#[doc = "Field `I2S23SEL` reader - clock source selection These bits are set and cleared by software to select the I2S23 clock source."]
pub type I2S23SEL_R = crate::FieldReader<I2S23SEL_A>;
#[doc = "clock source selection These bits are set and cleared by software to select the I2S23 clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2S23SEL_A {
    #[doc = "0: System clock selected as I2S23 clock"]
    B_0X0 = 0,
    #[doc = "1: PLL Q clock selected as I2S23 clock"]
    B_0X1 = 1,
    #[doc = "2: Clock provided on I2S_CKIN pin is selected as I2S23 clock"]
    B_0X2 = 2,
    #[doc = "3: HSI16 clock selected as I2S23 clock."]
    B_0X3 = 3,
}
impl From<I2S23SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2S23SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2S23SEL_A {
    type Ux = u8;
}
impl I2S23SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S23SEL_A {
        match self.bits {
            0 => I2S23SEL_A::B_0X0,
            1 => I2S23SEL_A::B_0X1,
            2 => I2S23SEL_A::B_0X2,
            3 => I2S23SEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2S23SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2S23SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2S23SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == I2S23SEL_A::B_0X3
    }
}
#[doc = "Field `I2S23SEL` writer - clock source selection These bits are set and cleared by software to select the I2S23 clock source."]
pub type I2S23SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, RCC_CCIPR_SPEC, 2, O, I2S23SEL_A>;
impl<'a, const O: u8> I2S23SEL_W<'a, O> {
    #[doc = "System clock selected as I2S23 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2S23SEL_A::B_0X0)
    }
    #[doc = "PLL Q clock selected as I2S23 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2S23SEL_A::B_0X1)
    }
    #[doc = "Clock provided on I2S_CKIN pin is selected as I2S23 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(I2S23SEL_A::B_0X2)
    }
    #[doc = "HSI16 clock selected as I2S23 clock."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(I2S23SEL_A::B_0X3)
    }
}
#[doc = "Field `FDCANSEL` reader - None"]
pub type FDCANSEL_R = crate::FieldReader;
#[doc = "Field `FDCANSEL` writer - None"]
pub type FDCANSEL_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CCIPR_SPEC, 2, O>;
#[doc = "Field `CLK48SEL` reader - 48 MHz clock source selection These bits are set and cleared by software to select the 48 MHz clock source used by USB device FS and RNG."]
pub type CLK48SEL_R = crate::FieldReader<CLK48SEL_A>;
#[doc = "48 MHz clock source selection These bits are set and cleared by software to select the 48 MHz clock source used by USB device FS and RNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK48SEL_A {
    #[doc = "0: HSI48 clock selected as 48 MHz clock"]
    B_0X0 = 0,
    #[doc = "2: PLL Q clock (PLL48M1CLK) selected as 48 MHz clock"]
    B_0X2 = 2,
    #[doc = "3: Reserved, must be kept at reset value"]
    B_0X3 = 3,
}
impl From<CLK48SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK48SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLK48SEL_A {
    type Ux = u8;
}
impl CLK48SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK48SEL_A> {
        match self.bits {
            0 => Some(CLK48SEL_A::B_0X0),
            2 => Some(CLK48SEL_A::B_0X2),
            3 => Some(CLK48SEL_A::B_0X3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CLK48SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CLK48SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CLK48SEL_A::B_0X3
    }
}
#[doc = "Field `CLK48SEL` writer - 48 MHz clock source selection These bits are set and cleared by software to select the 48 MHz clock source used by USB device FS and RNG."]
pub type CLK48SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CCIPR_SPEC, 2, O, CLK48SEL_A>;
impl<'a, const O: u8> CLK48SEL_W<'a, O> {
    #[doc = "HSI48 clock selected as 48 MHz clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CLK48SEL_A::B_0X0)
    }
    #[doc = "PLL Q clock (PLL48M1CLK) selected as 48 MHz clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(CLK48SEL_A::B_0X2)
    }
    #[doc = "Reserved, must be kept at reset value"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(CLK48SEL_A::B_0X3)
    }
}
#[doc = "Field `ADC12SEL` reader - ADC1/2 clock source selection These bits are set and cleared by software to select the clock source used by the ADC interface."]
pub type ADC12SEL_R = crate::FieldReader<ADC12SEL_A>;
#[doc = "ADC1/2 clock source selection These bits are set and cleared by software to select the clock source used by the ADC interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC12SEL_A {
    #[doc = "0: No clock selected"]
    B_0X0 = 0,
    #[doc = "1: PLL P clock selected as ADC1/2 clock"]
    B_0X1 = 1,
    #[doc = "2: System clock selected as ADC1/2 clock"]
    B_0X2 = 2,
}
impl From<ADC12SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC12SEL_A {
    type Ux = u8;
}
impl ADC12SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC12SEL_A> {
        match self.bits {
            0 => Some(ADC12SEL_A::B_0X0),
            1 => Some(ADC12SEL_A::B_0X1),
            2 => Some(ADC12SEL_A::B_0X2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADC12SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADC12SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ADC12SEL_A::B_0X2
    }
}
#[doc = "Field `ADC12SEL` writer - ADC1/2 clock source selection These bits are set and cleared by software to select the clock source used by the ADC interface."]
pub type ADC12SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RCC_CCIPR_SPEC, 2, O, ADC12SEL_A>;
impl<'a, const O: u8> ADC12SEL_W<'a, O> {
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADC12SEL_A::B_0X0)
    }
    #[doc = "PLL P clock selected as ADC1/2 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADC12SEL_A::B_0X1)
    }
    #[doc = "System clock selected as ADC1/2 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ADC12SEL_A::B_0X2)
    }
}
#[doc = "Field `ADC345SEL` reader - ADC3/4/5 clock source selection These bits are set and cleared by software to select the clock source used by the ADC345 interface."]
pub type ADC345SEL_R = crate::FieldReader<ADC345SEL_A>;
#[doc = "ADC3/4/5 clock source selection These bits are set and cleared by software to select the clock source used by the ADC345 interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC345SEL_A {
    #[doc = "0: No clock selected"]
    B_0X0 = 0,
    #[doc = "1: PLL P clock selected as ADC345 clock"]
    B_0X1 = 1,
    #[doc = "2: System clock selected as ADC3/4/5 clock"]
    B_0X2 = 2,
    #[doc = "3: Reserved."]
    B_0X3 = 3,
}
impl From<ADC345SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC345SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC345SEL_A {
    type Ux = u8;
}
impl ADC345SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC345SEL_A {
        match self.bits {
            0 => ADC345SEL_A::B_0X0,
            1 => ADC345SEL_A::B_0X1,
            2 => ADC345SEL_A::B_0X2,
            3 => ADC345SEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADC345SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADC345SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ADC345SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ADC345SEL_A::B_0X3
    }
}
#[doc = "Field `ADC345SEL` writer - ADC3/4/5 clock source selection These bits are set and cleared by software to select the clock source used by the ADC345 interface."]
pub type ADC345SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, RCC_CCIPR_SPEC, 2, O, ADC345SEL_A>;
impl<'a, const O: u8> ADC345SEL_W<'a, O> {
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADC345SEL_A::B_0X0)
    }
    #[doc = "PLL P clock selected as ADC345 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADC345SEL_A::B_0X1)
    }
    #[doc = "System clock selected as ADC3/4/5 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ADC345SEL_A::B_0X2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(ADC345SEL_A::B_0X3)
    }
}
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection This bit is set and cleared by software to select the USART1 clock source."]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection This bit is set and cleared by software to select the USART2 clock source."]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection This bit is set and cleared by software to select the USART3 clock source."]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - UART4 clock source selection This bit is set and cleared by software to select the UART4 clock source."]
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - UART5 clock source selection These bits are set and cleared by software to select the UART5 clock source."]
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection These bits are set and cleared by software to select the LPUART1 clock source."]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection These bits are set and cleared by software to select the I2C1 clock source."]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection These bits are set and cleared by software to select the I2C2 clock source."]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection These bits are set and cleared by software to select the I2C3 clock source."]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection These bits are set and cleared by software to select the LPTIM1 clock source."]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - clock source selection These bits are set and cleared by software to select the SAI clock source."]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - clock source selection These bits are set and cleared by software to select the I2S23 clock source."]
    #[inline(always)]
    pub fn i2s23sel(&self) -> I2S23SEL_R {
        I2S23SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - None"]
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection These bits are set and cleared by software to select the 48 MHz clock source used by USB device FS and RNG."]
    #[inline(always)]
    pub fn clk48sel(&self) -> CLK48SEL_R {
        CLK48SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - ADC1/2 clock source selection These bits are set and cleared by software to select the clock source used by the ADC interface."]
    #[inline(always)]
    pub fn adc12sel(&self) -> ADC12SEL_R {
        ADC12SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - ADC3/4/5 clock source selection These bits are set and cleared by software to select the clock source used by the ADC345 interface."]
    #[inline(always)]
    pub fn adc345sel(&self) -> ADC345SEL_R {
        ADC345SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection This bit is set and cleared by software to select the USART1 clock source."]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<0> {
        USART1SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection This bit is set and cleared by software to select the USART2 clock source."]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<2> {
        USART2SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection This bit is set and cleared by software to select the USART3 clock source."]
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<4> {
        USART3SEL_W::new(self)
    }
    #[doc = "Bits 6:7 - UART4 clock source selection This bit is set and cleared by software to select the UART4 clock source."]
    #[inline(always)]
    #[must_use]
    pub fn uart4sel(&mut self) -> UART4SEL_W<6> {
        UART4SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - UART5 clock source selection These bits are set and cleared by software to select the UART5 clock source."]
    #[inline(always)]
    #[must_use]
    pub fn uart5sel(&mut self) -> UART5SEL_W<8> {
        UART5SEL_W::new(self)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection These bits are set and cleared by software to select the LPUART1 clock source."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<10> {
        LPUART1SEL_W::new(self)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection These bits are set and cleared by software to select the I2C1 clock source."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<12> {
        I2C1SEL_W::new(self)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection These bits are set and cleared by software to select the I2C2 clock source."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<14> {
        I2C2SEL_W::new(self)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection These bits are set and cleared by software to select the I2C3 clock source."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<16> {
        I2C3SEL_W::new(self)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection These bits are set and cleared by software to select the LPTIM1 clock source."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<18> {
        LPTIM1SEL_W::new(self)
    }
    #[doc = "Bits 20:21 - clock source selection These bits are set and cleared by software to select the SAI clock source."]
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<20> {
        SAI1SEL_W::new(self)
    }
    #[doc = "Bits 22:23 - clock source selection These bits are set and cleared by software to select the I2S23 clock source."]
    #[inline(always)]
    #[must_use]
    pub fn i2s23sel(&mut self) -> I2S23SEL_W<22> {
        I2S23SEL_W::new(self)
    }
    #[doc = "Bits 24:25 - None"]
    #[inline(always)]
    #[must_use]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<24> {
        FDCANSEL_W::new(self)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection These bits are set and cleared by software to select the 48 MHz clock source used by USB device FS and RNG."]
    #[inline(always)]
    #[must_use]
    pub fn clk48sel(&mut self) -> CLK48SEL_W<26> {
        CLK48SEL_W::new(self)
    }
    #[doc = "Bits 28:29 - ADC1/2 clock source selection These bits are set and cleared by software to select the clock source used by the ADC interface."]
    #[inline(always)]
    #[must_use]
    pub fn adc12sel(&mut self) -> ADC12SEL_W<28> {
        ADC12SEL_W::new(self)
    }
    #[doc = "Bits 30:31 - ADC3/4/5 clock source selection These bits are set and cleared by software to select the clock source used by the ADC345 interface."]
    #[inline(always)]
    #[must_use]
    pub fn adc345sel(&mut self) -> ADC345SEL_W<30> {
        ADC345SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripherals independent clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ccipr](index.html) module"]
pub struct RCC_CCIPR_SPEC;
impl crate::RegisterSpec for RCC_CCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ccipr::R](R) reader structure"]
impl crate::Readable for RCC_CCIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ccipr::W](W) writer structure"]
impl crate::Writable for RCC_CCIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_CCIPR to value 0"]
impl crate::Resettable for RCC_CCIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
