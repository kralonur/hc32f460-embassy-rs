#[doc = "Register `DR0` reader"]
pub type R = crate::R<Dr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DR0\n\nYou can [`read`](crate::Reg::read) this register and get [`dr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr0Spec;
impl crate::RegisterSpec for Dr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dr0::R`](R) reader structure"]
impl crate::Readable for Dr0Spec {}
#[doc = "`reset()` method sets DR0 to value 0"]
impl crate::Resettable for Dr0Spec {}
