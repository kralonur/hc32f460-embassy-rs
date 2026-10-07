#[doc = "Register `DR15` reader"]
pub type R = crate::R<Dr15Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR15\n\nYou can [`read`](crate::Reg::read) this register and get [`dr15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr15Spec;
impl crate::RegisterSpec for Dr15Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr15::R`](R) reader structure"]
impl crate::Readable for Dr15Spec {}
#[doc = "`reset()` method sets DR15 to value 0"]
impl crate::Resettable for Dr15Spec {}
