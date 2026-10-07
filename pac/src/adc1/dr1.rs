#[doc = "Register `DR1` reader"]
pub type R = crate::R<Dr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR1\n\nYou can [`read`](crate::Reg::read) this register and get [`dr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr1Spec;
impl crate::RegisterSpec for Dr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr1::R`](R) reader structure"]
impl crate::Readable for Dr1Spec {}
#[doc = "`reset()` method sets DR1 to value 0"]
impl crate::Resettable for Dr1Spec {}
