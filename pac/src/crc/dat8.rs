#[doc = "Register `DAT8` reader"]
pub type R = crate::R<Dat8Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT8\n\nYou can [`read`](crate::Reg::read) this register and get [`dat8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat8Spec;
impl crate::RegisterSpec for Dat8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat8::R`](R) reader structure"]
impl crate::Readable for Dat8Spec {}
#[doc = "`reset()` method sets DAT8 to value 0"]
impl crate::Resettable for Dat8Spec {}
