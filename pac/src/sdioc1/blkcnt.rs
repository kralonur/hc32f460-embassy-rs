#[doc = "Register `BLKCNT` reader"]
pub type R = crate::R<BlkcntSpec>;
#[doc = "Register `BLKCNT` writer"]
pub type W = crate::W<BlkcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc BLKCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`blkcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blkcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkcntSpec;
impl crate::RegisterSpec for BlkcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`blkcnt::R`](R) reader structure"]
impl crate::Readable for BlkcntSpec {}
#[doc = "`write(|w| ..)` method takes [`blkcnt::W`](W) writer structure"]
impl crate::Writable for BlkcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLKCNT to value 0"]
impl crate::Resettable for BlkcntSpec {}
