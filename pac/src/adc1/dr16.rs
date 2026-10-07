#[doc = "Register `DR16` reader"]
pub type R = crate::R<Dr16Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR16\n\nYou can [`read`](crate::Reg::read) this register and get [`dr16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr16Spec;
impl crate::RegisterSpec for Dr16Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr16::R`](R) reader structure"]
impl crate::Readable for Dr16Spec {}
#[doc = "`reset()` method sets DR16 to value 0"]
impl crate::Resettable for Dr16Spec {}
