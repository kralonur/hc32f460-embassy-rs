#[doc = "Register `TT_WTRIG` reader"]
pub type R = crate::R<TtWtrigSpec>;
#[doc = "Register `TT_WTRIG` writer"]
pub type W = crate::W<TtWtrigSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc TT_WTRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`tt_wtrig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tt_wtrig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TtWtrigSpec;
impl crate::RegisterSpec for TtWtrigSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tt_wtrig::R`](R) reader structure"]
impl crate::Readable for TtWtrigSpec {}
#[doc = "`write(|w| ..)` method takes [`tt_wtrig::W`](W) writer structure"]
impl crate::Writable for TtWtrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TT_WTRIG to value 0xffff"]
impl crate::Resettable for TtWtrigSpec {
    const RESET_VALUE: u16 = 0xffff;
}
