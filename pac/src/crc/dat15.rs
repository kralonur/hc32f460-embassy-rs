#[doc = "Register `DAT15` reader"]
pub type R = crate::R<Dat15Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT15\n\nYou can [`read`](crate::Reg::read) this register and get [`dat15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat15Spec;
impl crate::RegisterSpec for Dat15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat15::R`](R) reader structure"]
impl crate::Readable for Dat15Spec {}
#[doc = "`reset()` method sets DAT15 to value 0"]
impl crate::Resettable for Dat15Spec {}
