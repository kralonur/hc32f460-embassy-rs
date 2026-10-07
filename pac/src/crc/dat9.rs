#[doc = "Register `DAT9` reader"]
pub type R = crate::R<Dat9Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT9\n\nYou can [`read`](crate::Reg::read) this register and get [`dat9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat9Spec;
impl crate::RegisterSpec for Dat9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat9::R`](R) reader structure"]
impl crate::Readable for Dat9Spec {}
#[doc = "`reset()` method sets DAT9 to value 0"]
impl crate::Resettable for Dat9Spec {}
