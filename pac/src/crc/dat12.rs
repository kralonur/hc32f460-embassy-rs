#[doc = "Register `DAT12` reader"]
pub type R = crate::R<Dat12Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT12\n\nYou can [`read`](crate::Reg::read) this register and get [`dat12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat12Spec;
impl crate::RegisterSpec for Dat12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat12::R`](R) reader structure"]
impl crate::Readable for Dat12Spec {}
#[doc = "`reset()` method sets DAT12 to value 0"]
impl crate::Resettable for Dat12Spec {}
