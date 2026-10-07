#[doc = "Register `DR7` reader"]
pub type R = crate::R<Dr7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR7\n\nYou can [`read`](crate::Reg::read) this register and get [`dr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr7Spec;
impl crate::RegisterSpec for Dr7Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr7::R`](R) reader structure"]
impl crate::Readable for Dr7Spec {}
#[doc = "`reset()` method sets DR7 to value 0"]
impl crate::Resettable for Dr7Spec {}
