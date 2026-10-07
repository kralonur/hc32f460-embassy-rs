#[doc = "Register `DAT28` reader"]
pub type R = crate::R<Dat28Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT28\n\nYou can [`read`](crate::Reg::read) this register and get [`dat28::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat28Spec;
impl crate::RegisterSpec for Dat28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat28::R`](R) reader structure"]
impl crate::Readable for Dat28Spec {}
#[doc = "`reset()` method sets DAT28 to value 0"]
impl crate::Resettable for Dat28Spec {}
