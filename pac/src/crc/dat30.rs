#[doc = "Register `DAT30` reader"]
pub type R = crate::R<Dat30Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT30\n\nYou can [`read`](crate::Reg::read) this register and get [`dat30::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat30Spec;
impl crate::RegisterSpec for Dat30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat30::R`](R) reader structure"]
impl crate::Readable for Dat30Spec {}
#[doc = "`reset()` method sets DAT30 to value 0"]
impl crate::Resettable for Dat30Spec {}
