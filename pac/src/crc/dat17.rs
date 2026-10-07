#[doc = "Register `DAT17` reader"]
pub type R = crate::R<Dat17Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT17\n\nYou can [`read`](crate::Reg::read) this register and get [`dat17::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat17Spec;
impl crate::RegisterSpec for Dat17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat17::R`](R) reader structure"]
impl crate::Readable for Dat17Spec {}
#[doc = "`reset()` method sets DAT17 to value 0"]
impl crate::Resettable for Dat17Spec {}
