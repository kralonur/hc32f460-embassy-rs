#[doc = "Register `MONDAR1` reader"]
pub type R = crate::R<Mondar1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc MONDAR1\n\nYou can [`read`](crate::Reg::read) this register and get [`mondar1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mondar1Spec;
impl crate::RegisterSpec for Mondar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mondar1::R`](R) reader structure"]
impl crate::Readable for Mondar1Spec {}
#[doc = "`reset()` method sets MONDAR1 to value 0"]
impl crate::Resettable for Mondar1Spec {}
