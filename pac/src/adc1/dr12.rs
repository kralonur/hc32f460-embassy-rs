#[doc = "Register `DR12` reader"]
pub type R = crate::R<Dr12Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR12\n\nYou can [`read`](crate::Reg::read) this register and get [`dr12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr12Spec;
impl crate::RegisterSpec for Dr12Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr12::R`](R) reader structure"]
impl crate::Readable for Dr12Spec {}
#[doc = "`reset()` method sets DR12 to value 0"]
impl crate::Resettable for Dr12Spec {}
