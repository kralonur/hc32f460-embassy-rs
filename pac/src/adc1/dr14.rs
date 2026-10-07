#[doc = "Register `DR14` reader"]
pub type R = crate::R<Dr14Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR14\n\nYou can [`read`](crate::Reg::read) this register and get [`dr14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr14Spec;
impl crate::RegisterSpec for Dr14Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr14::R`](R) reader structure"]
impl crate::Readable for Dr14Spec {}
#[doc = "`reset()` method sets DR14 to value 0"]
impl crate::Resettable for Dr14Spec {}
