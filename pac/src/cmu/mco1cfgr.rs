#[doc = "Register `MCO1CFGR` reader"]
pub type R = crate::R<Mco1cfgrSpec>;
#[doc = "Register `MCO1CFGR` writer"]
pub type W = crate::W<Mco1cfgrSpec>;
#[doc = "Field `MCOSEL` reader - desc MCOSEL"]
pub type McoselR = crate::FieldReader;
#[doc = "Field `MCOSEL` writer - desc MCOSEL"]
pub type McoselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCODIV` reader - desc MCODIV"]
pub type McodivR = crate::FieldReader;
#[doc = "Field `MCODIV` writer - desc MCODIV"]
pub type McodivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCOEN` reader - desc MCOEN"]
pub type McoenR = crate::BitReader;
#[doc = "Field `MCOEN` writer - desc MCOEN"]
pub type McoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - desc MCOSEL"]
    #[inline(always)]
    pub fn mcosel(&self) -> McoselR {
        McoselR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - desc MCODIV"]
    #[inline(always)]
    pub fn mcodiv(&self) -> McodivR {
        McodivR::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - desc MCOEN"]
    #[inline(always)]
    pub fn mcoen(&self) -> McoenR {
        McoenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc MCOSEL"]
    #[inline(always)]
    pub fn mcosel(&mut self) -> McoselW<'_, Mco1cfgrSpec> {
        McoselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc MCODIV"]
    #[inline(always)]
    pub fn mcodiv(&mut self) -> McodivW<'_, Mco1cfgrSpec> {
        McodivW::new(self, 4)
    }
    #[doc = "Bit 7 - desc MCOEN"]
    #[inline(always)]
    pub fn mcoen(&mut self) -> McoenW<'_, Mco1cfgrSpec> {
        McoenW::new(self, 7)
    }
}
#[doc = "desc MCO1CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`mco1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mco1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mco1cfgrSpec;
impl crate::RegisterSpec for Mco1cfgrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mco1cfgr::R`](R) reader structure"]
impl crate::Readable for Mco1cfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`mco1cfgr::W`](W) writer structure"]
impl crate::Writable for Mco1cfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCO1CFGR to value 0"]
impl crate::Resettable for Mco1cfgrSpec {}
