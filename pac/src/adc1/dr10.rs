#[doc = "Register `DR10` reader"]
pub type R = crate::R<Dr10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR10\n\nYou can [`read`](crate::Reg::read) this register and get [`dr10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr10Spec;
impl crate::RegisterSpec for Dr10Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr10::R`](R) reader structure"]
impl crate::Readable for Dr10Spec {}
#[doc = "`reset()` method sets DR10 to value 0"]
impl crate::Resettable for Dr10Spec {}
