#[doc = "Register `POTRC` reader"]
pub type R = crate::R<PotrcSpec>;
#[doc = "Register `POTRC` writer"]
pub type W = crate::W<PotrcSpec>;
#[doc = "Field `POT13` reader - desc POT13"]
pub type Pot13R = crate::BitReader;
#[doc = "Field `POT13` writer - desc POT13"]
pub type Pot13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT14` reader - desc POT14"]
pub type Pot14R = crate::BitReader;
#[doc = "Field `POT14` writer - desc POT14"]
pub type Pot14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT15` reader - desc POT15"]
pub type Pot15R = crate::BitReader;
#[doc = "Field `POT15` writer - desc POT15"]
pub type Pot15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13 - desc POT13"]
    #[inline(always)]
    pub fn pot13(&self) -> Pot13R {
        Pot13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc POT14"]
    #[inline(always)]
    pub fn pot14(&self) -> Pot14R {
        Pot14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc POT15"]
    #[inline(always)]
    pub fn pot15(&self) -> Pot15R {
        Pot15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - desc POT13"]
    #[inline(always)]
    pub fn pot13(&mut self) -> Pot13W<'_, PotrcSpec> {
        Pot13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc POT14"]
    #[inline(always)]
    pub fn pot14(&mut self) -> Pot14W<'_, PotrcSpec> {
        Pot14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc POT15"]
    #[inline(always)]
    pub fn pot15(&mut self) -> Pot15W<'_, PotrcSpec> {
        Pot15W::new(self, 15)
    }
}
#[doc = "desc POTRC\n\nYou can [`read`](crate::Reg::read) this register and get [`potrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`potrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PotrcSpec;
impl crate::RegisterSpec for PotrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`potrc::R`](R) reader structure"]
impl crate::Readable for PotrcSpec {}
#[doc = "`write(|w| ..)` method takes [`potrc::W`](W) writer structure"]
impl crate::Writable for PotrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POTRC to value 0"]
impl crate::Resettable for PotrcSpec {}
