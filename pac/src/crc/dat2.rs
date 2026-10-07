#[doc = "Register `DAT2` reader"]
pub type R = crate::R<Dat2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT2\n\nYou can [`read`](crate::Reg::read) this register and get [`dat2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat2Spec;
impl crate::RegisterSpec for Dat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat2::R`](R) reader structure"]
impl crate::Readable for Dat2Spec {}
#[doc = "`reset()` method sets DAT2 to value 0"]
impl crate::Resettable for Dat2Spec {}
