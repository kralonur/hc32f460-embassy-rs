#[doc = "Register `POERC` reader"]
pub type R = crate::R<PoercSpec>;
#[doc = "Register `POERC` writer"]
pub type W = crate::W<PoercSpec>;
#[doc = "Field `POUTE13` reader - desc POUTE13"]
pub type Poute13R = crate::BitReader;
#[doc = "Field `POUTE13` writer - desc POUTE13"]
pub type Poute13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE14` reader - desc POUTE14"]
pub type Poute14R = crate::BitReader;
#[doc = "Field `POUTE14` writer - desc POUTE14"]
pub type Poute14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE15` reader - desc POUTE15"]
pub type Poute15R = crate::BitReader;
#[doc = "Field `POUTE15` writer - desc POUTE15"]
pub type Poute15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13 - desc POUTE13"]
    #[inline(always)]
    pub fn poute13(&self) -> Poute13R {
        Poute13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc POUTE14"]
    #[inline(always)]
    pub fn poute14(&self) -> Poute14R {
        Poute14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc POUTE15"]
    #[inline(always)]
    pub fn poute15(&self) -> Poute15R {
        Poute15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - desc POUTE13"]
    #[inline(always)]
    pub fn poute13(&mut self) -> Poute13W<'_, PoercSpec> {
        Poute13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc POUTE14"]
    #[inline(always)]
    pub fn poute14(&mut self) -> Poute14W<'_, PoercSpec> {
        Poute14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc POUTE15"]
    #[inline(always)]
    pub fn poute15(&mut self) -> Poute15W<'_, PoercSpec> {
        Poute15W::new(self, 15)
    }
}
#[doc = "desc POERC\n\nYou can [`read`](crate::Reg::read) this register and get [`poerc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poerc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PoercSpec;
impl crate::RegisterSpec for PoercSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`poerc::R`](R) reader structure"]
impl crate::Readable for PoercSpec {}
#[doc = "`write(|w| ..)` method takes [`poerc::W`](W) writer structure"]
impl crate::Writable for PoercSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POERC to value 0"]
impl crate::Resettable for PoercSpec {}
