#[doc = "Register `TBUF` reader"]
pub type R = crate::R<TbufSpec>;
#[doc = "Register `TBUF` writer"]
pub type W = crate::W<TbufSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc TBUF\n\nYou can [`read`](crate::Reg::read) this register and get [`tbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbufSpec;
impl crate::RegisterSpec for TbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbuf::R`](R) reader structure"]
impl crate::Readable for TbufSpec {}
#[doc = "`write(|w| ..)` method takes [`tbuf::W`](W) writer structure"]
impl crate::Writable for TbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBUF to value 0"]
impl crate::Resettable for TbufSpec {}
