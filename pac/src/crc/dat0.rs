#[doc = "Register `DAT0` reader"]
pub type R = crate::R<Dat0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT0\n\nYou can [`read`](crate::Reg::read) this register and get [`dat0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat0Spec;
impl crate::RegisterSpec for Dat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat0::R`](R) reader structure"]
impl crate::Readable for Dat0Spec {}
#[doc = "`reset()` method sets DAT0 to value 0"]
impl crate::Resettable for Dat0Spec {}
