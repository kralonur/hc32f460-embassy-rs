#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `TXBA` reader - desc TXBA"]
pub type TxbaR = crate::BitReader;
#[doc = "Field `RXBA` reader - desc RXBA"]
pub type RxbaR = crate::BitReader;
#[doc = "Field `TXBE` reader - desc TXBE"]
pub type TxbeR = crate::BitReader;
#[doc = "Field `TXBF` reader - desc TXBF"]
pub type TxbfR = crate::BitReader;
#[doc = "Field `RXBE` reader - desc RXBE"]
pub type RxbeR = crate::BitReader;
#[doc = "Field `RXBF` reader - desc RXBF"]
pub type RxbfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc TXBA"]
    #[inline(always)]
    pub fn txba(&self) -> TxbaR {
        TxbaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXBA"]
    #[inline(always)]
    pub fn rxba(&self) -> RxbaR {
        RxbaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TXBE"]
    #[inline(always)]
    pub fn txbe(&self) -> TxbeR {
        TxbeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TXBF"]
    #[inline(always)]
    pub fn txbf(&self) -> TxbfR {
        TxbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RXBE"]
    #[inline(always)]
    pub fn rxbe(&self) -> RxbeR {
        RxbeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RXBF"]
    #[inline(always)]
    pub fn rxbf(&self) -> RxbfR {
        RxbfR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0x14"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x14;
}
