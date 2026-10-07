#[doc = "Register `DAT5` reader"]
pub type R = crate::R<Dat5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT5\n\nYou can [`read`](crate::Reg::read) this register and get [`dat5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat5Spec;
impl crate::RegisterSpec for Dat5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat5::R`](R) reader structure"]
impl crate::Readable for Dat5Spec {}
#[doc = "`reset()` method sets DAT5 to value 0"]
impl crate::Resettable for Dat5Spec {}
