#[doc = "Register `DAT18` reader"]
pub type R = crate::R<Dat18Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT18\n\nYou can [`read`](crate::Reg::read) this register and get [`dat18::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat18Spec;
impl crate::RegisterSpec for Dat18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat18::R`](R) reader structure"]
impl crate::Readable for Dat18Spec {}
#[doc = "`reset()` method sets DAT18 to value 0"]
impl crate::Resettable for Dat18Spec {}
