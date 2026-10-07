#[doc = "Register `DR5` reader"]
pub type R = crate::R<Dr5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR5\n\nYou can [`read`](crate::Reg::read) this register and get [`dr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr5Spec;
impl crate::RegisterSpec for Dr5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr5::R`](R) reader structure"]
impl crate::Readable for Dr5Spec {}
#[doc = "`reset()` method sets DR5 to value 0"]
impl crate::Resettable for Dr5Spec {}
