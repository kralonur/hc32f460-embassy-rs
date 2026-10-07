#[doc = "Register `TT_TRIG` reader"]
pub type R = crate::R<TtTrigSpec>;
#[doc = "Register `TT_TRIG` writer"]
pub type W = crate::W<TtTrigSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc TT_TRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`tt_trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tt_trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TtTrigSpec;
impl crate::RegisterSpec for TtTrigSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tt_trig::R`](R) reader structure"]
impl crate::Readable for TtTrigSpec {}
#[doc = "`write(|w| ..)` method takes [`tt_trig::W`](W) writer structure"]
impl crate::Writable for TtTrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TT_TRIG to value 0"]
impl crate::Resettable for TtTrigSpec {}
