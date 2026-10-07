#[doc = "Register `DAT4` reader"]
pub type R = crate::R<Dat4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT4\n\nYou can [`read`](crate::Reg::read) this register and get [`dat4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat4Spec;
impl crate::RegisterSpec for Dat4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat4::R`](R) reader structure"]
impl crate::Readable for Dat4Spec {}
#[doc = "`reset()` method sets DAT4 to value 0"]
impl crate::Resettable for Dat4Spec {}
