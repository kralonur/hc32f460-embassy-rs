#[doc = "Register `DR4` reader"]
pub type R = crate::R<Dr4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR4\n\nYou can [`read`](crate::Reg::read) this register and get [`dr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr4Spec;
impl crate::RegisterSpec for Dr4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr4::R`](R) reader structure"]
impl crate::Readable for Dr4Spec {}
#[doc = "`reset()` method sets DR4 to value 0"]
impl crate::Resettable for Dr4Spec {}
