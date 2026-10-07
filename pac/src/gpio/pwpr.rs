#[doc = "Register `PWPR` reader"]
pub type R = crate::R<PwprSpec>;
#[doc = "Register `PWPR` writer"]
pub type W = crate::W<PwprSpec>;
#[doc = "Field `WE` reader - desc WE"]
pub type WeR = crate::BitReader;
#[doc = "Field `WE` writer - desc WE"]
pub type WeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP` writer - desc WP"]
pub type WpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - desc WE"]
    #[inline(always)]
    pub fn we(&self) -> WeR {
        WeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc WE"]
    #[inline(always)]
    pub fn we(&mut self) -> WeW<'_, PwprSpec> {
        WeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - desc WP"]
    #[inline(always)]
    pub fn wp(&mut self) -> WpW<'_, PwprSpec> {
        WpW::new(self, 8)
    }
}
#[doc = "desc PWPR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwprSpec;
impl crate::RegisterSpec for PwprSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pwpr::R`](R) reader structure"]
impl crate::Readable for PwprSpec {}
#[doc = "`write(|w| ..)` method takes [`pwpr::W`](W) writer structure"]
impl crate::Writable for PwprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWPR to value 0"]
impl crate::Resettable for PwprSpec {}
