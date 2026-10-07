#[doc = "Register `DAT7` reader"]
pub type R = crate::R<Dat7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT7\n\nYou can [`read`](crate::Reg::read) this register and get [`dat7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat7Spec;
impl crate::RegisterSpec for Dat7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat7::R`](R) reader structure"]
impl crate::Readable for Dat7Spec {}
#[doc = "`reset()` method sets DAT7 to value 0"]
impl crate::Resettable for Dat7Spec {}
