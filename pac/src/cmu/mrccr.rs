#[doc = "Register `MRCCR` reader"]
pub type R = crate::R<MrccrSpec>;
#[doc = "Register `MRCCR` writer"]
pub type W = crate::W<MrccrSpec>;
#[doc = "Field `MRCSTP` reader - desc MRCSTP"]
pub type MrcstpR = crate::BitReader;
#[doc = "Field `MRCSTP` writer - desc MRCSTP"]
pub type MrcstpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc MRCSTP"]
    #[inline(always)]
    pub fn mrcstp(&self) -> MrcstpR {
        MrcstpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc MRCSTP"]
    #[inline(always)]
    pub fn mrcstp(&mut self) -> MrcstpW<'_, MrccrSpec> {
        MrcstpW::new(self, 0)
    }
}
#[doc = "desc MRCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`mrccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrccrSpec;
impl crate::RegisterSpec for MrccrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mrccr::R`](R) reader structure"]
impl crate::Readable for MrccrSpec {}
#[doc = "`write(|w| ..)` method takes [`mrccr::W`](W) writer structure"]
impl crate::Writable for MrccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MRCCR to value 0x80"]
impl crate::Resettable for MrccrSpec {
    const RESET_VALUE: u8 = 0x80;
}
