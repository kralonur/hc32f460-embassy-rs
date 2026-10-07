#[doc = "Register `DAT20` reader"]
pub type R = crate::R<Dat20Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT20\n\nYou can [`read`](crate::Reg::read) this register and get [`dat20::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat20Spec;
impl crate::RegisterSpec for Dat20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat20::R`](R) reader structure"]
impl crate::Readable for Dat20Spec {}
#[doc = "`reset()` method sets DAT20 to value 0"]
impl crate::Resettable for Dat20Spec {}
