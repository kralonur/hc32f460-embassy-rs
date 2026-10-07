#[doc = "Register `MONDAR3` reader"]
pub type R = crate::R<Mondar3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc MONDAR3\n\nYou can [`read`](crate::Reg::read) this register and get [`mondar3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mondar3Spec;
impl crate::RegisterSpec for Mondar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mondar3::R`](R) reader structure"]
impl crate::Readable for Mondar3Spec {}
#[doc = "`reset()` method sets MONDAR3 to value 0"]
impl crate::Resettable for Mondar3Spec {}
