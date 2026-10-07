#[doc = "Register `DAT21` reader"]
pub type R = crate::R<Dat21Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT21\n\nYou can [`read`](crate::Reg::read) this register and get [`dat21::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat21Spec;
impl crate::RegisterSpec for Dat21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat21::R`](R) reader structure"]
impl crate::Readable for Dat21Spec {}
#[doc = "`reset()` method sets DAT21 to value 0"]
impl crate::Resettable for Dat21Spec {}
