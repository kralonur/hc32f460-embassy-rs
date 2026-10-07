#[doc = "Register `INTSTAT1` reader"]
pub type R = crate::R<Intstat1Spec>;
#[doc = "Field `TC` reader - desc TC"]
pub type TcR = crate::FieldReader;
#[doc = "Field `BTC` reader - desc BTC"]
pub type BtcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - desc TC"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - desc BTC"]
    #[inline(always)]
    pub fn btc(&self) -> BtcR {
        BtcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "desc INTSTAT1\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intstat1Spec;
impl crate::RegisterSpec for Intstat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat1::R`](R) reader structure"]
impl crate::Readable for Intstat1Spec {}
#[doc = "`reset()` method sets INTSTAT1 to value 0"]
impl crate::Resettable for Intstat1Spec {}
