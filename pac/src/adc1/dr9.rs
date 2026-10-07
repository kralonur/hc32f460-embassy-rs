#[doc = "Register `DR9` reader"]
pub type R = crate::R<Dr9Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR9\n\nYou can [`read`](crate::Reg::read) this register and get [`dr9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr9Spec;
impl crate::RegisterSpec for Dr9Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr9::R`](R) reader structure"]
impl crate::Readable for Dr9Spec {}
#[doc = "`reset()` method sets DR9 to value 0"]
impl crate::Resettable for Dr9Spec {}
