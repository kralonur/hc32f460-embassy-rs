#[doc = "Register `DAT1` reader"]
pub type R = crate::R<Dat1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT1\n\nYou can [`read`](crate::Reg::read) this register and get [`dat1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat1Spec;
impl crate::RegisterSpec for Dat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat1::R`](R) reader structure"]
impl crate::Readable for Dat1Spec {}
#[doc = "`reset()` method sets DAT1 to value 0"]
impl crate::Resettable for Dat1Spec {}
