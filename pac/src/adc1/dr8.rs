#[doc = "Register `DR8` reader"]
pub type R = crate::R<Dr8Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR8\n\nYou can [`read`](crate::Reg::read) this register and get [`dr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr8Spec;
impl crate::RegisterSpec for Dr8Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr8::R`](R) reader structure"]
impl crate::Readable for Dr8Spec {}
#[doc = "`reset()` method sets DR8 to value 0"]
impl crate::Resettable for Dr8Spec {}
