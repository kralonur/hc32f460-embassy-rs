#[doc = "Register `DAT29` reader"]
pub type R = crate::R<Dat29Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT29\n\nYou can [`read`](crate::Reg::read) this register and get [`dat29::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat29Spec;
impl crate::RegisterSpec for Dat29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat29::R`](R) reader structure"]
impl crate::Readable for Dat29Spec {}
#[doc = "`reset()` method sets DAT29 to value 0"]
impl crate::Resettable for Dat29Spec {}
