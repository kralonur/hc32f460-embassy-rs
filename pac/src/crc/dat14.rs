#[doc = "Register `DAT14` reader"]
pub type R = crate::R<Dat14Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT14\n\nYou can [`read`](crate::Reg::read) this register and get [`dat14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat14Spec;
impl crate::RegisterSpec for Dat14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat14::R`](R) reader structure"]
impl crate::Readable for Dat14Spec {}
#[doc = "`reset()` method sets DAT14 to value 0"]
impl crate::Resettable for Dat14Spec {}
