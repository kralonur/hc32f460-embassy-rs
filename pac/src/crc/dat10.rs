#[doc = "Register `DAT10` reader"]
pub type R = crate::R<Dat10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DAT10\n\nYou can [`read`](crate::Reg::read) this register and get [`dat10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat10Spec;
impl crate::RegisterSpec for Dat10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat10::R`](R) reader structure"]
impl crate::Readable for Dat10Spec {}
#[doc = "`reset()` method sets DAT10 to value 0"]
impl crate::Resettable for Dat10Spec {}
